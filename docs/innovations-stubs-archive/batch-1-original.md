# Spiral Innovations — Stubs for the Ten New Ideas

**Status:** design stubs (2026-06-15), user-approved
**Author:** implementer agent
**Phase context:** M4 first sprint complete; this is a backlog of
**proposed** future work, not committed.
**Purpose:** capture ten novel ideas in concrete enough detail that
the maintainer can pick which to invest in. Each idea has a
"novelty check" (honest prior-art assessment) and a "build cost"
estimate, so a future decision can be made without re-deriving the
research.

---

## How to read this doc

Each idea has the same structure:

1. **Pitch** — one paragraph, what the user/developer gets.
2. **What's novel** — the specific thing that has no prior art (or
   whose combination is new).
3. **What's prior art** — honest assessment of components that
   already exist. Same honesty as the M4 audit (`docs/audit-sprint-m4.md`).
4. **Build cost** — engineer-months estimate.
5. **M-month target** — which phase this would land in.
6. **Open questions** — what we don't know yet.
7. **Depends on** — which existing Spiral crate(s) it builds on.

The novelty classifications come from the M4 audit methodology:
- **Truly novel** = no shipped browser does this combination.
- **Partially novel** = the components exist; Spiral's combination
  is new.
- **Configuration** = a sound engineering choice, not a uniqueness
  claim.

---

## 1. The "Wound Lattice" — Server-Push That Compiles, Not Streams

**Pitch.** When you visit a Spiral-optimised site, the server has
already pre-run the response through Spiral's actual rendering
pipeline (DOM, layout, CSS, JS) and ships the result as a signed
binary blob. The browser unzips, validates the signature, and
**skips 80% of the work** that a cold page load would otherwise
do. The DOM is already there. The layout is already there. The CSS
is already resolved. The JS is already compiled. The page appears
essentially *instantly* — not just loaded, but already laid out,
already styled, already in its final position.

**What's novel.** No browser ships a *compiled response* that the
client unpacks. HTTP/2 push ships raw bytes. Speculation Rules
prefetch but don't pre-render. Chrome's BFCache keeps the rendering
state but only for back/forward navigation. The Wound Lattice
inverts Bet 4 (the persistent renderer / warm cache): instead of
mmap'ing a previously-rendered tab, we mmap a server-prepared
response. The serialisation format is the same; the source is
different.

**What's prior art.** The serialisation format is novel; the
*idea* of pre-computing a page on the server has prior art:

- **Cloudflare Workers + HTML Rewriter** (2017) — server-side HTML
  rewriting, but not pre-rendering.
- **Vercel/Next.js server components** (2020) — server-pre-rendered
  HTML, but only the HTML; layout/CSS/JS still happens client-side.
- **Isomorphic/Universal JS** (2013) — server runs the same JS as
  the client, but ships the result, not a serialised engine state.
- **Mozilla's RLBox / wasm-sandboxing** (2020) — different
  problem (sandboxing), but uses WASM as the deployment format.
  Spiral could use WASM as the compiled-response format too.

**Build cost.** 4–6 engineer-months. Includes: stable binary
serialisation for DOM, layout, CSS, JS bytecode; signature scheme
(Spiral-team-controlled key); server-side rendering API
(`spiral-fmt` and `spiral-gyre` exposed for server use); client
deserialiser with fast-path mmap; policy for cache invalidation.

**M-month target.** M30+ (when Gyre is stable and Vortex has a
bytecode VM).

**Open questions.**
- Whose key signs the compiled response? Spiral team, site owner,
  or both?
- What's the freshness model? TTL? ETag? Re-sign-on-mutation?
- Does this create a fingerprinting surface (your browser shipped
  a serialised engine state — does that leak version info)?
- What's the bandwidth tradeoff? A serialised DOM is smaller than
  raw HTML, but the layout + JS is bigger.

**Depends on.** `spiral-dom` (serialise), `spiral-gyre` (serialise
layout), `spiral-css` (serialise cascade), `spiral-vortex`
(serialise bytecode), `spiral-fmt` (server-side rendering), the
Bet 4 persistent-renderer work.

---

## 2. Provenance Tracking — A Read Audit Trail Inside the Browser

**Pitch.** Every piece of data the browser stores (cookies,
localStorage, IndexedDB, cached responses, form values) is signed
with a chain of keys — origin key + user key + notary key — that
the browser maintains in encrypted form. Six months later, the
user can ask: "what does this site know about me?" and see a
*graph* of data flows, decrypted on demand, exportable as a
signed forensic package.

**What's novel.** Browsers have private-browsing mode (don't
store). Browsers have storage permissions (control storage). No
browser ships *forensic-grade read receipts* for what the browser
stored and from whom. The combination of cryptographic chains +
selective decryption + user-facing audit is genuinely new.

**What's prior art.**
- **Certificate Transparency** (2013) — append-only public logs
  of TLS certificates. Different domain (certificates, not
  storage), but the same cryptographic-chain idea.
- **W3C Verifiable Claims** (2022) — cryptographically-signed
  assertions. We could use this data format for the audit export.
- **GnuPG signed email audit** — same model, different media.

**Build cost.** 3–4 engineer-months. Includes: key derivation and
storage; per-write signing (must be cheap, or every localStorage
set is slow); per-storage-entry metadata; the user-facing graph
viewer; the export format.

**M-month target.** M18+ (when storage is real).

**Open questions.**
- Where does the user key live? TPM? iCloud Keychain? User-typed
  passphrase? Different threat models, different choices.
- What if the user loses the key? The audit trail is permanently
  encrypted. Acceptable, or a UX problem?
- Does this conflict with private-browsing mode? (Probably yes.
  Private mode should not maintain the chain.)
- Is this a privacy improvement or a privacy regression? It
  depends on whether the user trusts themselves more or less
  than they trust the browser.

**Depends on.** `spiral-storage` (planned), a new `spiral-audit`
crate (suggested in `docs/design-filter-rule-model.md:5.7`).

---

## 3. Sectional Reload — Surgical Refresh of Only the Broken Part

**Pitch.** When part of a page breaks, the user can right-click
that section and choose "Reload this section." The browser
identifies the DOM subtree, the JS modules that own it, the CSS
rules that style it, and the storage entries those modules use.
It reloads *just that section* — re-fetches its data, re-runs its
JS, re-lays it out, re-paints it. The rest of the page stays
interactive. "Reload this ad" without reloading the article.
"Reload this form" without retyping the others.

**What's novel.** Chrome's "soft reload" and Firefox's
"Ctrl+Shift+R" are whole-page. Chrome's "scoped reload" extension
API exists but is fragile. No browser exposes this as a
first-class user feature, especially not for static HTML. The
feasibility depends on Vortex's origin-tagged GC (per-arena
collection is cheap) and the DOM being arena-allocated (Bet 1).

**What's prior art.**
- **Vite, webpack HMR** — Hot Module Replacement is a
  developer-time feature for SPAs, not a user-time feature.
- **React Fast Refresh** — same model, framework-specific.
- **LiveView (Phoenix)** — server-driven DOM, but no client-side
  reload primitive.
- **Chrome's "Edit & Reload" DevTools feature** — reloads a
  single element in DevTools, but not user-facing.

**Build cost.** 2–3 engineer-months. Includes: section-to-module
graph (which JS modules own which DOM nodes); the reload
transaction (snapshot, fetch, re-init, swap); the rollback
mechanism; the user-facing UX (right-click, context menu).

**M-month target.** M36+ (needs mature DOM bindings and a stable
JS-module graph).

**Open questions.**
- How do we identify which JS module "owns" a section? DOM
  attributes? Frameworks? Heuristics?
- What if reloading a section breaks another section that depends
  on it? Need a dependency graph.
- What about CSS scoping? A section's styles might leak.
- Does this work for iframes? Probably needs to be opt-in per
  iframe.

**Depends on.** `spiral-dom` (arena allocation), `spiral-vortex`
(per-module isolation), `spiral-gyre` (incremental layout).

---

## 4. Type-Verified URL — Phishing Protection by Compiler

**Pitch.** A URL of type `Bank<yourbank>` can only redirect to
URLs of the same type. A URL of type `SearchEngine<Google>` can
only redirect to URLs of the same type. Cross-type redirects
require an explicit user-approved cast. The browser ships with a
curated, signed registry of URL types. A malicious site literally
*cannot phish* if the type system refuses to send the form.

**What's novel.** Browsers have EV certificates (2006, mostly
abandoned). Browsers have phishing blacklists (reactive, always
behind). No browser uses *type theory* to make phishing
structurally impossible. This is the only "no, really, this can't
happen" defense in the arsenal.

**What's prior art.**
- **HSTS / HPKP** — pin a site's identity to a key, not a type.
  Different mechanism, similar effect.
- **WebAuthn** — public-key auth that resists phishing. Different
  problem (authentication, not URL navigation).
- **CSP `form-action`** — restricts where forms can POST. Half
  the idea (the allowlist); missing the type system.
- **Information-flow type systems** (Volpano, Smith, others) —
  academic prior art for the type-theory idea.

**Build cost.** 4–5 engineer-months. Includes: type registry
(who maintains? who signs? how is it updated?); type inference
for URLs (what type is `https://yourbank.com/login`?); cross-type
redirect handling; the user-facing UX for "URL type changed, do
you want to proceed?".

**M-month target.** M36+ (needs network-layer cooperation).

**Open questions.**
- Who maintains the registry? Spiral team? Community? Site owners
  self-register?
- What's the upgrade story? Sites change URLs. A bank migrates
  to a new domain. The type registry needs to handle re-keying.
- How do you bootstrap trust? First visit to a new bank, you
  don't have a type yet. Need an out-of-band verification.
- Does this conflict with the URL-as-data model? Some sites
  construct URLs dynamically. The type system has to handle that.

**Depends on.** `spiral-network` (URL inspection), `spiral-filter`
(the "form-action" enforcement point), a new `spiral-types`
crate (or extend `spiral-context`).

---

## 5. Sandbox Sandboxing — Sandboxes for Sandboxes

**Pitch.** The renderer process is itself split into
capability-restricted compartments:

- A **DOM compartment** that can read/write the DOM tree but
  cannot open files, cannot make network requests, cannot talk
  to the GPU.
- A **CSS compartment** that can read the DOM and write the
  style cascade but cannot execute JS.
- A **JS compartment** (Vortex) that can read/write the DOM and
  run scripts but cannot open files.
- A **Network compartment** (in a separate process anyway) that
  can only talk to a fixed set of origins.
- A **GPU compartment** that can only draw, not read.

A bug in Vortex's interpreter cannot read files. A bug in the
CSS parser cannot execute JS.

**What's novel.** Chrome's site isolation is process-level.
Spiral's compartment isolation is *finer-grained than process*
(in-process) but *coarser than thread*. No browser I've found
does this. It's a natural extension of Bet 1 (shared-everything
multi-process) but applied within the renderer.

**What's prior art.**
- **Chromium Mojo / process-per-renderer** — coarser-grained
  (process boundary).
- **FreeBSD Capsicum capability mode** — kernel-level, not
  user-space Rust types.
- **seL4 capability types** — kernel-level, verified.
- **WASI preview 2 capability model** — in-process WASM sandbox.
  Most directly applicable; Spiral could use WASI to model
  compartments.

**Build cost.** 2–3 engineer-months. Mostly extending `spiral-context`'s
existing capability-typed API surface with compartment-level
scopes. The infrastructure is already in place (Bet 1).

**M-month target.** M25–M30 (when `spiral-context`'s runtime
lands).

**Open questions.**
- What's the IPC cost between compartments? Each call is a
  capability check; need to ensure it's cheap.
- How do compartments share data? The DOM compartment hands a
  handle to the JS compartment; the handle is typed.
- What about WASM modules inside a page? Each WASM module is a
  compartment by default (existing WASM sandboxing).

**Depends on.** `spiral-context` (capability types, `Context`).
This is essentially "Bet 1 but finer-grained."

---

## 6. Reactive Extensions as a First-Class Browser API

**Pitch.** WebExtensions are imperative and event-callback-based.
Spiral's extension API is reactive: every browser event is a
stream. The extension is a graph of stream operators. The browser
provides a visual debugger for these streams.

```javascript
browser.tabs.query({ url: "*://*.youtube.com/*" })
  .flatMap(tab => browser.tabs.stream(tab))
  .filter(tab => tab.url.includes("/watch"))
  .flatMap(tab => browser.tabs.executeScript(tab.id,
      "return document.querySelector('video').duration"))
  .filter(duration => duration > 3600)
  .subscribe(({ tab, duration }) => {
    browser.action.setBadgeText({ tabId: tab.id, text: "🎬" });
  });
```

**What's novel.** RxJS exists as a library. No browser ships a
*built-in* reactive extension API. The visual debugger is also
novel — Chrome DevTools shows you the DOM tree, the network
waterfall, the JS heap, but not *the extension's stream graph*.

**What's prior art.**
- **RxJS** (2015) — the reactive library for JS. Same operators
  (`map`, `filter`, `flatMap`, etc.) but as a library, not a
  built-in.
- **XUL/XBL event streams** (Firefox 2003–2017) — declarative
  event handling in browser chrome, but not exposed to
  extensions.
- **SwiftUI / Jetpack Compose** — declarative UI frameworks
  with similar reactive semantics. Different domain (UI, not
  extensions).
- **Flow-based programming** (J. Paul Morrison, 1970s) — the
  academic ancestor.

**Build cost.** 3–4 engineer-months. Includes: the reactive
runtime; the operator library; the WebIDL bindings; the
DevTools-style stream visualizer; the migration path for
existing WebExtensions.

**M-month target.** M42+ (M4 if we want to validate the API
shape early).

**Open questions.**
- Is RxJS the right model, or should we use a different
  reactive paradigm (signals, observables, streams)?
- How do streams interact with backpressure? Some browser events
  fire at 60 Hz; the extension should be able to throttle.
- Cold or hot streams? Each has tradeoffs.
- Compatibility with existing WebExtensions? Drop-in or new
  parallel API?

**Depends on.** `spiral-extensions` (new crate), `spiral-ui`
(DevTools).

---

## 7. Layout Streams — Continuous Layout, Frame-by-Frame

**Pitch.** Layout is a stream, not a batch. Gyre publishes layout
as a stream of `{ dirty_region, layout_box }` events. The painter
subscribes. The screen updates as the stream emits,
frame-by-frame, throttled to the display refresh rate. Gyre
doesn't re-lay the whole tree on every change. It tracks the
*minimal* invalidation set. With Bet 4's persistent renderer, it
can checkpoint layout state. With Bet 1's shared style cache, it
can share style computation across origins.

**What's novel.** All layout engines are batched. Blink, Gecko,
WebKit, Servo, Ladybird all run layout as a "do everything, then
paint" loop. No engine runs layout as a continuous, event-driven
stream. The stream interface is novel; the underlying incremental
layout algorithm is not.

**What's prior art.**
- **Blink's `LayoutShift` and `LayoutInvalidation`** — Blink
  already tracks invalidation. It just publishes the result as
  a batch, not a stream.
- **Servo's parallel layout** — different problem (parallelism).
- **React's reconciler** — same continuous-stream idea, but
  applied to virtual DOM, not layout.
- **CRDT / operational transform** — continuous merge algorithms
  in collaborative editing; same continuous-publish idea.

**Build cost.** 3–4 engineer-months. Includes: the stream
abstraction over Gyre; the dirty-region tracking; the painter
subscription; the frame-throttling; the rollback on invalidation
during in-flight paint.

**M-month target.** M25+ (when Gyre is stable).

**Open questions.**
- How do we avoid stutter on a fast invalidation burst (e.g.
  typing in a textarea)? Backpressure.
- How do we handle layout that depends on viewport size?
  Streaming invalidation per viewport change.
- What about fonts loading mid-stream? Layout depends on
  metrics; metrics change when fonts arrive.

**Depends on.** `spiral-gyre` (incremental layout), `spiral-paint`
(consumer of the stream).

---

## 8. WASM as a Cross-Process RPC — Type-Safe IPC for Free

**Pitch.** `spiral-ipc` exposes WebAssembly System Interface
(WASI) bindings to renderer code. A renderer process calls
`wasi:ipc/send` to send a typed message to the browser process.
The browser process receives it as a WASM function call. No JSON,
no bincode, no protobuf. No code generation, no bindings. Just
types.

**What's novel.** WASM is used for compute, not IPC. WASI is used
for system access, not for browser-internal messaging. The idea
of using WASM as a cross-process RPC layer in a browser is not
in any shipped browser.

**What's prior art.**
- **WASI preview 1 / preview 2** — the same capability model
  Spiral would use for IPC.
- **WASM Component Model** (2023) — types as the unit of
  composition across module boundaries. Directly applicable.
- **gRPC** — typed RPC, but text-based protobuf, not WASM.
- **Cap'n Proto** — zero-copy typed RPC, similar spirit.
- **Chromium Mojo** — bespoke IPC, hand-written bindings.
- **Firefox IPDL** — same.
- **WebKit CoreIPC** — same.

**Build cost.** 3–5 engineer-months. Includes: the WASM runtime
(we'd use `wasmtime` or `wasmer`); the WASI bindings for IPC;
the type definitions (in `.wit`); the integration with the
existing `spiral-ipc`; the migration path for existing IPC
messages.

**M-month target.** M36+ (when IPC is critical-path and the
WASM Component Model is stable).

**Open questions.**
- WASM Component Model is still evolving. Should we wait for
  the 1.0 spec, or commit to preview 2?
- What's the per-call overhead of WASM calls? Need to
  benchmark.
- Can we use WASM for the existing `IPCMessage` enum, or do we
  need a parallel WASM-typed path?
- Licensing: `wasmtime` and `wasmer` are both Apache-2.0/MIT;
  compatible with MPL-2.0.

**Depends on.** `spiral-ipc` (existing transport), a new
`spiral-wasm-ipc` crate or extension.

---

## 9. Tab Provenance Graph — Show the User Where a Tab Came From

**Pitch.** Every tab has a provenance record that shows its
lineage: "this tab was opened by JavaScript on news.ycombinator.com,
which was opened by a link from example.com, which was opened by
my address bar." The user can see the chain in a side panel.
"I have 47 tabs. 12 were opened by me. 31 were opened by
JavaScript. 4 were opened by an extension. 0 I have no idea."

**What's novel.** Chrome's tab-history shows you *this* tab's
navigation history (the URL chain within a tab), not the *chain
of how you got to this tab* across tab boundaries. No browser
tracks cross-tab provenance.

**What's prior art.**
- **Chrome's "Tab Groups"** — visual grouping, no provenance.
- **Firefox's "Tree Style Tab"** — hierarchical, but visually
  only.
- **Chrome's "Recently Closed"** — linear history, not a graph.
- **Session restore in any browser** — saves the *state* of
  tabs but not the *provenance*.

**Build cost.** 1–2 engineer-months. Includes: per-tab provenance
record; cross-tab linking (when a tab opens another tab, record
the link); the side-panel UI; storage of the graph.

**M-month target.** M12+ (when tab management is real).

**Open questions.**
- How long is the provenance retained? Forever? Session-only?
  User-configurable?
- Does the graph have a privacy dimension? Knowing the chain
  *could* be sensitive ("I got to this medical site from this
  porn site"). Need a redacted-view option.
- What about `window.open()` from an iframe? Does the new tab's
  provenance include the iframe's origin?

**Depends on.** `spiral-browser` (tab management), `spiral-ui`
(side panel).

---

## 10. Self-Patching Bugs — The Browser Fixes Its Own Security Issues

**Pitch.** When a security researcher finds a bug, they don't
wait for a release — they ship a signed patch as a WASM module.
The browser loads it, validates the signature against a
Spiral-maintained key, and applies it. The patch is a WASM
function that takes the buggy function's arguments and returns a
safe result. "Browser has 0 unpatched known security issues (3
patches applied this week)."

**What's novel.** Hot-patching has existed in OS kernels
(KSplice, kpatch) and in some game engines. No browser ships
*user-installable, cryptographically-signed, in-process security
patches* that can be applied without a full release. The
cryptographic chain and the WASM-format combination are new.

**What's prior art.**
- **KSplice / kpatch / livepatch** — kernel hot-patches.
- **eBPF** — kernel-level code that can be hot-loaded.
- **Hot-code-replace in Erlang / BEAM** — language runtime
  hot-code-swap.
- **Cloudflare's "Wait, that's a vuln? Let me patch it
  globally"** — server-side hot-patching at the edge.
- **Chrome's "binary patching"** — not exposed to users;
  vendor-only.

**Build cost.** 5–7 engineer-months. Includes: the patch
distribution system; the WASM sandbox; the signature
verification; the patch application API; the user-facing
"applied patches" panel; the rollback mechanism; the audit log
of who applied what patch when.

**M-month target.** M48+ (v0.1 ship date). Needs all subsystems
to be stable first.

**Open questions.**
- Who has the signing key? Spiral team only? A small group of
  trusted researchers?
- What's the rollback story? If a patch breaks a site, can the
  user disable it?
- Does this create a "two-version" problem? Some users have
  patch A, some have patch B. Sites might depend on a
  specific patch.
- What if a researcher ships a malicious patch? The signature
  protects against this, but the trust model is critical.

**Depends on.** All subsystems (this is end-game work), a
WASM runtime (`wasmtime` or `wasmer`), a key management
infrastructure, a patch distribution server.

---

## Cross-cutting Notes

### Trust and key management

Several ideas (#1 Wound Lattice, #2 Provenance, #4 Type-Verified
URL, #10 Self-Patching) involve a Spiral-controlled signing key.
This is a significant trust decision. The right model is probably:

- **Spiral team holds the master key.**
- **Security researchers can request signing authority** for
  critical patches.
- **A user-configurable trust level** lets power users accept
  additional keys (e.g. their employer's internal Spiral
  signing key).
- **All signed artifacts are auditable** — the user can see
  every signed thing that has affected their browser.

### Build sequencing

These ten ideas cluster into roughly four phases:

| Phase | Ideas |
|-------|-------|
| **Phase 2 (M4–9)** | None — too ambitious for the foundation phase. |
| **Phase 3 (M10–24)** | #5 Sandbox Sandboxing (extends Bet 1), #9 Tab Provenance Graph (low-cost) |
| **Phase 4 (M25–42)** | #3 Sectional Reload, #4 Type-Verified URLs, #6 Reactive Extensions API, #7 Layout Streams |
| **Phase 5 (M43–60)** | #1 Wound Lattice, #2 Provenance Tracking, #8 WASM-as-IPC, #10 Self-Patching |

#9 is the cheapest and can land earliest as a "good UX win"
during Phase 2 (the tab management phase).

### What the M4 audit implies for these

The M4 audit established that the techniques Spiral uses (branded
types, capability tokens, mark-sweep GC, ABP parsing) are
well-known prior art re-implemented in Spiral-native code. **The
same approach applies to these ten ideas**: research the
components, design the combination, write Spiral-native code,
attribute prior art honestly.

---

## SSOT Links

- [`docs/active_context.md`](active_context.md) — live sprint state
- [`docs/progress_ledger.md`](progress_ledger.md) — change log
- [`docs/audit-sprint-m4.md`](audit-sprint-m4.md) — M4 originality
  audit (methodology applied here)
- [`docs/architecture-shared-everything.md`](architecture-shared-everything.md) —
  Bet 1 (relates to #5)
- [`docs/design-filter-rule-model.md`](design-filter-rule-model.md) —
  relates to #4 (form-action enforcement)
- [`docs/design-capability-types.md`](design-capability-types.md) —
  relates to #5
- [`docs/design-vortex-heap.md`](design-vortex-heap.md) —
  relates to #3 (per-arena reload)
- [`ROADMAP.md`](../ROADMAP.md) — phase plan
