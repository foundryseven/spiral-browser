# Architecture — Shared-Everything Multi-Process (SEM)

**Status:** design pass complete (2026-06-14), user-approved
**Author:** implementer agent
**Phase context:** Phase 2 first sprint (M4) — type system + Vortex isolate
abstraction land now; runtime lands in M25–M36

---

## 1. The Problem This Solves

Every shipped browser has picked one of two points on the memory/speed
curve:

- **Memory-heavy + fast** — Chromium. Multi-process, pre-warm everything,
  copy-on-write fork, deep caching. Result: 4–8 GB for a normal session,
  60fps, instant tabs.
- **Memory-light + slow** — early Firefox, early Ladybird, NetSurf.
  Single-process, lazy everything. Result: <500 MB, but 200ms tab
  switches and janky scrolling.

**No browser in history has been memory-light AND fast at scale.** Servo
got close with parallelism (parallel layout, parallel paint) but never
shipped at Chromium scale. This is the unsolved problem.

The big 3 cannot solve it: their architecture is locked in. Chromium's
process-per-tab model is the *source* of its memory cost. Firefox's
content process model has the same problem. Safari is the closest, with
its "Web Content" service per tab, but it still duplicates WebKit
internals per process.

Spiral is new. Spiral can pick a different architecture. This document
describes that architecture.

---

## 2. The Architecture

### 2.1 The Mental Model

**One renderer process per browser instance. N typed-isolated contexts
inside it.**

A *context* is the per-origin bundle: DOM, CSSOM, JS globals, layout
tree, cookies, storage, IndexedDB. Each origin has exactly one context.
A context is a value with a *type*; the type system enforces what the
context can do.

A *renderer process* is the shared bundle: the Vortex interpreter and
heap, the Gyre layout engine, the HTML/CSS parser, the font system, the
glyph atlas, the image decoders, the standard library, the network
filters. The renderer process owns one Vortex heap, one Gyre layout
engine, one parser, one font system — and serves N contexts out of
those shared resources.

### 2.2 What Is Shared, What Is Per-Origin

| Resource | Per-origin (in the context) | Per-process (shared) |
|----------|------------------------------|----------------------|
| DOM | yes | no |
| CSSOM | yes | no |
| JS globals | yes | no |
| JS execution stack | yes (logical) | no |
| Vortex interpreter bytecode | no | yes |
| Vortex heap | shared, with origin-tagged allocations | yes |
| Gyre layout engine | no | yes |
| Gyre layout tree | yes (per origin) | no |
| Gyre style cache (CSS rules) | no | yes (keyed by rule hash) |
| HTML parser | no | yes (re-entrant) |
| CSS parser | no | yes |
| Font system | no | yes |
| Glyph atlas | no | yes |
| Image decoders | no | yes |
| Network filter | no | yes |
| Standard library bindings | no | yes |
| Cookies, storage, IndexedDB | yes | no |

The shared things are *typed* — they are API surfaces in Rust, not raw
pointers. A context cannot reach into the shared Vortex heap and read
another origin's strings, because the type system does not let it.

### 2.3 The Capability-Typed API Surface

A *capability* is a token that grants access to a specific resource.
Capabilities are *unforgeable* — they are values whose constructors are
private to the module that grants them. A `File` capability can only
be constructed by the filesystem module; a `Socket` capability can only
be constructed by the network module.

A context's API surface is a set of capabilities. The default context
has *no* filesystem capability, *no* network capability, *no* raw-pointer
capability. It can only call the typed APIs we expose: `read_dom`,
`call_vortex`, `layout_query`, etc.

```rust
// Pseudocode — illustrative, not yet implemented

/// A context handle. Branded with its origin.
pub struct Context {
    origin: Origin,
    _phantom: PhantomData<()>,  // not Send / not Sync
}

impl Context {
    /// Construct a new context for an origin. Only callable by the
    /// browser process when opening a tab.
    pub(crate) fn new(origin: Origin) -> Self { ... }

    /// Run a script in this context's Vortex isolate.
    pub fn run_script(&self, src: &str) -> Result<Value, ScriptError> { ... }

    /// Query the layout tree at a point (for hit testing).
    pub fn hit_test(&self, point: Point) -> Option<NodeId> { ... }
}

/// Filesystem capability. Only constructable by the FS module.
pub struct File { ... }
impl File {
    pub(crate) fn open(path: &Path) -> Result<Self, FsError> { ... }
}

/// The default context has no File. The user cannot open /etc/passwd
/// at all, because File is not in the capability set.
```

This is **capability-based security** in the classical sense (cf. Capsicum,
seL4, CloudABI). It is the same security model the Unix caps system was
designed around, except the enforcement is in the Rust type system, not
in the kernel.

### 2.4 The Process Model: Default vs. Escalation

**Default:** single renderer process, N typed contexts. 99% of sites
run in this mode. The capability types are the security boundary.

**Escalation:** per-origin "isolation mode" toggle. If the user has
marked `bank.com` as "isolated," that origin's context runs in a
*separate* renderer process, with full OS-level sandboxing
(Landlock+seccomp on Linux, Seatbelt on macOS, Restricted Token on
Windows). The user gets Chromium-class isolation for the sites that
need it, without paying for it on every tab.

This is the honest middle ground the user asked for. The capability
types are *stronger* than Ladybird's flat address space. The OS-level
sandbox is *as strong* as Chromium's, on demand.

### 2.5 The Vortex Isolate Abstraction

Vortex's `Isolate` is the *logical* JS context. It has its own globals,
its own call stack, its own GC roots. Multiple isolates live inside one
Vortex *heap* in the renderer process.

```rust
pub struct Isolate {
    globals: HandleMap<GlobalId, Value>,
    stack: Vec<StackFrame>,
    gc_roots: Vec<GcRoot>,
    origin: Origin,
}
```

Allocations are origin-tagged. The GC traces per-origin roots and only
collects unreachable origin-tagged allocations. This is what makes
multi-origin in one heap safe: an origin's GC cannot free another
origin's data.

The interpreter and bytecode VM are *not* per-isolate. The bytecode
executor is a function `(isolate: &Isolate, instr: &Bytecode) -> ()`.
Multiple isolates share the executor code. The bytecode cache is per-
process.

### 2.6 The Gyre Layout Engine

Gyre is per-process. Layout trees are per-origin. The layout *engine*
(its algorithms, its caches, its style-rule matcher) is shared.

Key design points:

- **One canonical layout tree per origin.** No "scroll tree" or
  "stacking context tree" duplication. Sticky and fixed positioning are
  resolved at paint time against the viewport, not pre-laid-out into
  separate trees.
- **Lazy box construction.** A box is only materialised when (i) it has
  visible effects or (ii) it is matched by a style rule. `display: none`
  subtrees cost nothing. Off-screen subtrees cost only the DOM, not the
  layout.
- **Arena-allocated.** One bump arena per document. No per-node
  allocation. Reclaim the whole arena on navigation.
- **Style cache shared across origins.** A third-party stylesheet
  loaded by 5 NYT tabs is parsed once, hashed, and matched from cache
  the next four times. Cache invalidates on document unload.

This is what gives Spiral its "minimum memory" property on the layout
side.

---

## 3. Why This Wins on Both Axes

### 3.1 Memory

A Chromium tab pays for:
- V8 isolate (~30 MB)
- DOM, CSSOM, layout tree (~50 MB for a heavy page)
- Browser process proxy and IPC stack (~10 MB)
- Skia graphics state (~20 MB)
- Network service state (~10 MB)
- Total: ~120 MB per tab

A Spiral tab pays for:
- Vortex isolate (logical only — heap is shared) (~5 MB bookkeeping)
- DOM, CSSOM, layout tree (lazy construction) (~30 MB for a heavy
  page with off-screen subtrees not materialised)
- Shared parser, layout engine, fonts (amortised across N tabs)
- Capability-type set (~constant)
- Total: ~35–50 MB per tab, on a process that holds 20 tabs in
  ~150 MB shared + 50 MB × 20 per-tab = ~1.1 GB total. Chromium pays
  ~120 × 20 = ~2.4 GB for the same workload.

**Estimated 2–3× memory reduction** vs. Chromium, with the same UX.

### 3.2 Speed

Shared resources warm once. The HTML parser is parsed once per
unique document, not once per tab. The Gyre style cache matches in
microseconds after the first hit. The font system loads each font
once. The image decoder decodes each unique image once.

The bytecode VM is shared. Cold start to "hello world" is the cost
of one VM init, not N VM inits. This is the *biggest* single speed
win on a cold-start measurement.

Incremental layout (every box is dirty-tracked) means re-layout is
the common case and is fast. The shared layout engine means hot
layouts (NYT header, Stripe checkout) have *already* been computed
by another origin and are in the cache.

**Estimated 1.5–2× speedup on warm-up, 1.2–1.5× on steady-state
scrolling** vs. Chromium. The wins are on cold-start and warm-path
shared work, not on peak frame rate (which is Vello's job).

---

## 4. Security Analysis

### 4.1 What the Capability Types Prevent

- **Filesystem escape:** a context has no `File` capability by default.
  It cannot open a file, period. The constructor is private to the
  filesystem module, which the context cannot reach.
- **Network exfiltration:** a context has no `Socket` capability by
  default. The network module grants a `Network` capability to the
  *page*, scoped to the page's origin and the user-granted permissions.
- **Memory corruption:** there is no `unsafe` in the public API. The
  shared arena is typed; contexts can only access origin-tagged
  allocations through handles, which are typed and checkable.
- **Cross-origin data leak:** the GC is origin-aware. An origin's GC
  roots cannot see another origin's allocations. The capability types
  cannot construct a handle for a different origin's data.

### 4.2 What the Capability Types Do NOT Prevent

- **Spectre-class attacks.** In-process data is in-process data. An
  attacker who controls a script can attempt side-channel reads
  against the shared arena. Mitigation: branch-prediction-resistant
  layout for secret-dependent data, no script-controlled pointer
  arithmetic, and (defense in depth) the optional OS-level sandbox
  for sensitive origins.
- **Vortex interpreter bugs.** A bug in the Vortex interpreter that
  allows a script to read arbitrary process memory is a *real* bug
  and is *real* damage. Mitigation: fuzz the interpreter (Month 6+),
  audit the bytecode verifier, and the OS-level sandbox is the
  fallback for sensitive origins.
- **Supply chain.** A malicious crate in the dependency tree has the
  same access it would have in any Rust program. Mitigation: minimum
  dependencies, cargo-audit in CI, periodic review. This is hygiene,
  not architecture.

### 4.3 The OS-Level Sandbox as Escalation

For origins the user has marked "isolated," the context runs in a
*separate* renderer process with the full OS-level sandbox:

- Linux: Landlock + seccomp-bpf. No execve, no filesystem outside
  the configured profile, no network outside the configured profile.
- macOS: Seatbelt profile. App sandbox, no fork, limited IPC.
- Windows: Restricted Token + Job Object. Low integrity, limited
  token groups.

The OS-level sandbox is *not* the default because it costs memory
(separate process per isolated origin) and complexity (separate
Vortex heap per isolated origin, no shared cache). It is the
*escalation* path.

### 4.4 Honest Comparison

| Browser | Default security | Best-case security | Memory cost of best case |
|---------|------------------|---------------------|--------------------------|
| Chromium | Per-process OS sandbox | Per-process OS sandbox | ~120 MB per tab |
| Firefox | Per-process OS sandbox | Per-process OS sandbox | ~80 MB per tab |
| Ladybird | Flat address space | Flat address space | ~50 MB per tab |
| **Spiral** | Capability types | Capability types + OS sandbox (per-origin) | ~50 MB default, ~120 MB escalated |

Spiral's *default* is stronger than Ladybird's best. Spiral's
*escalation* is as strong as Chromium's. The user chooses.

---

## 5. Implementation Phasing

### M4–M6 (Phase 2, ongoing) — Type system + Isolate abstraction

- `spiral-context` crate skeleton. `Context`, `Origin`, `Capability`
  trait, brand types.
- `spiral-vortex`: `Isolate` API designed and prototyped. Logical
  isolates, origin-tagged heap allocations, per-origin GC roots.
- `spiral-gyre`: confirm layout engine is process-singleton-friendly.
- A toy demo: one process, two contexts, one shared font system. The
  two contexts cannot see each other's DOM. Fuzz the capability
  boundary.

### M7–M24 (Phase 2–3) — Build the engines to SEM-friendly

- Vortex tree-walker (M4–9) and bytecode VM (M10–24) designed to
  run multiple isolates per heap. No code change required at the
  call-site level, but the *internals* are written with origin
  tagging from day one.
- Gyre block (M7–8), flex (M10–11), grid (M13–14) all written with
  the shared style cache. The first WPT fixture for shared cache
  hits is a unit test.
- The `spiral-context` API is exercised by `spiral-fmt::html` parsing
  into `spiral-dom` documents, which are then attached to a `Context`.

### M25–M36 (Phase 3) — The runtime

- `spiral-context` becomes the runtime, not just the type system.
  A real `BrowserShell` runs with one renderer process, N contexts.
- Vortex `Isolate::spawn()` is wired to context creation. Per-origin
  heap tagging is enforced.
- Gyre's style cache is exposed as a process-wide service.
- A `spiral-sandbox` (renamed role) layer adds per-origin OS-level
  escalation. Users can toggle it per origin.

### M36+ (Phase 4) — Polish and the warm cache

- Persistent renderer (Bet 4) lands on top of the SEM runtime. The
  Vortex heap is checkpointed to a memory-mapped file; the layout
  tree is mmap'd; the DOM hash is stored.
- The shared style cache survives across tab closes.
- The escalation path's OS sandbox profile is tuned for each
  platform.

---

## 6. The Cleverness, Restated

The big 3 cannot do this because they cannot change their process
model. Chromium has 15 years of process-per-tab assumption baked
into its rendering pipeline. Firefox has the same. Safari has it
across its Web Content service boundary.

Spiral is new. Spiral can build the type system *now* — in M4 —
even if the runtime lands in M25. The cost of building it now is
a few weeks of careful API design. The cost of building it later
is a rewrite of the entire engine.

This is what "smart and clever" means in practice: a 2–3× memory
win and a 1.5–2× speedup from a structural decision that costs
nothing to commit to early.

---

## 7. Open Questions (to resolve in M4–M6)

1. **Brand types in stable Rust.** Rust does not have branded types
   in stable. The current plan is to use `#[doc(hidden)]` modules
   with private constructors and `PhantomData<*const ()>` for
   `!Send + !Sync`. We should evaluate `#[fundamental]` niche
   approaches. *Owner: spiral-context skeleton reviewer.*

2. **Origin-tagged GC.** Mark-sweep with origin tags requires the
   heap to know the origin of every allocation. The current Vortex
   heap is generic; the M4–M6 work is to add origin tagging. *Owner:
   Vortex spike.*

3. **Shared style cache invalidation.** When does a cached style
   rule expire? On document unload? On `document.styleSheets`
   mutation? On a global "flush" event? *Owner: Gyre design.*

4. **Escalation toggling UX.** How does a user mark `bank.com` as
   "isolated"? A site settings panel? A first-run prompt for known
   banking domains? A heuristic that auto-escalates pages with
   `autocomplete=cc-number`? *Owner: UI design, M4 backlog.*

5. **Spectre mitigations in Rust.** What's the cost of constant-time
   branches for the relevant secret-dependent code paths? Are the
   existing `subtle` crate primitives sufficient? *Owner: security
   review, M6.*

---

## 8. SSOT Links

- [`docs/active_context.md`](active_context.md) — live sprint state
- [`docs/progress_ledger.md`](progress_ledger.md) — change log
- [`docs/system_architecture.md`](system_architecture.md) — architecture deltas
- [`docs/plans/iteration-options.md`](plans/iteration-options.md) — dependency triage
- `ROADMAP.md` — phase plan (Group → Phase → Step → Packet index)
- `ARCHITECTURE.md` — canonical architecture (one-page index)

## 9. Forks (decisions that bend the shared-everything bet)

The shared-everything architecture is a single
architectural bet, but it has had to be adjusted
in two places since the design was written:

- **Fork 1 — process-global JS context per
  origin (1.6.1 GC rewrite).** Originally
  the Vortex heap was a single global arena;
  the 1.6.1 GC rewrite ([`vortex-heap.md`](vortex-heap.md))
  split it into per-origin `OriginArena` chunks
  so the GC can collect per-origin. This is
  structural refinement, not a bet change.

- **Fork 2 — process-global `FilterHook`
  (ADR 0005).** The shared-everything bet
  says the network stack calls into
  `FilterHook::decide` for every outbound
  request. ADR 0005 moved the three types
  (`FilterHook` / `Decision` / `Party`) from
  `spiral-filter` to `spiral-core` so the
  network crate could depend on them without
  pulling in the whole HTML/CSS/rule-DSL dep
  graph. See
  [`0005-filter-hook-architecture.md`](../decisions/0005-filter-hook-architecture.md).
  This is a dep-arrow correction, not a bet
  change.

Both forks preserve the architectural bet.
