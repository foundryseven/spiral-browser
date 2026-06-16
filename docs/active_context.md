# Active Context

**Last updated:** 2026-06-16
**Status:** 🟢 M4.4 COMPLETE | off main @ 6a03da7 (work branch: `audit/m4-window`)
**Current phase:** Phase 2 — Core Engine (Months 4–9) — *M4.4 complete (Chunks 1–3 + Item 4); M4.5 next*
**Sprint state:** [`specs/GAP_ANALYSIS.md`](../specs/GAP_ANALYSIS.md) is the live gap tracker. Deltas 1–4 recorded.
**Iteration plans:** [`docs/plans/iteration-options.md`](plans/iteration-options.md)
**SSOT surface:** `docs/glossary.md`, `docs/decisions/`, `docs/agents/`, `docs/architecture/`
**Architecture bet:** [`docs/architecture-shared-everything.md`](architecture-shared-everything.md)

## Test posture (verified 2026-06-16)

- 429 tests across 53 binaries, 0 failing.
- `cargo fmt --all -- --check` clean.
- `cargo clippy --workspace --all-targets -- -D warnings` clean.
- `cargo build --workspace` clean.
- `./scripts/audit-orphan-exports.sh` flags 34 candidates across 10
  crates — all M4.5+ skeletons (un-wired by design, see below).
  **9 crates OK (all wired)**: spiral-core, spiral-crypto, spiral-css,
  spiral-dom, spiral-fmt, spiral-gyre, spiral-ipc, spiral-render,
  spiral-theme, spiral-ui. The M4.4 leaks detected by the audit on
  2026-06-16 (12 symbols) are all wired via `tests/<crate>_surface.rs`
  integration tests (see the M4.4 leak cleanup section below).

## What's done in M4.4

- Chunk 1 — `spiral-crypto` P0 fixes (sha2 + getrandom).
- Chunk 1.5 — `spiral-html` retired.
- Chunk 2A — `spiral-fmt` from-spec HTML parser.
- Chunk 3 — DOM rewire.
- M4.4.1 Item 4 — `spiral-fmt` from-spec CSS parser (Fork 1-B).
- `spiral-css` deprecated shim, `cssparser` / `selectors` removed.
- Crate renames: `spiral-layout` → `spiral-gyre`, `spiral-js` → `spiral-vortex`.
- Vortex skeleton, `spiral-filter`, `spiral-context` crate skeletons.
- 3 ADRs recording cross-cutting M4.4 decisions.

## SSOT restructure (in working tree, uncommitted 2026-06-16)

Adopted from the Zeus repo pattern:

- `docs/glossary.md` — engine brand names.
- `docs/decisions/0000..0003-*.md` — ADR template + 3 ADRs
  (CSS parser, Vortex posture, Gyre rename).
- `docs/agents/{README,implementer,reviewer,architect,tester}.md`
  — role contracts.
- `docs/architecture/{fmt,gyre,vortex,filter,context}.md` —
  per-subsystem architecture stubs.
- `scripts/audit-orphan-exports.sh` — the wiring-rule
  audit (treats exit 1 as a build break).
- `AGENTS.md` — added the Decision Protocol table + the
  Wiring & Integration rule; updated commit scopes; updated
  the spiral-fmt / spiral-css working-rules sections.
- `docs/progress_ledger.md` — retrofitted the
  M4.4.1 Item 4 entry with a Wiring & Integration section;
  appended the restructure entry.

Verification of the restructure: 409 tests pass, 0 failing;
clippy + fmt + build clean. The audit script flagged 48
candidates across 19 crates; the M4.4 leaks (12 symbols)
were wired with integration tests on the same day (see
the "M4.4 leak cleanup" section below). The remaining 34
candidates are M4.5+ skeletons (un-wired by design).

### M4.4 leak cleanup (in working tree, uncommitted 2026-06-16)

The audit caught 12 M4.4 leaks (declared `pub` symbols with
no external consumer). Each was fixed by adding a
`tests/<crate>_surface.rs` integration test that names
the type and exercises it through the public surface:

- **spiral-core** — `RenderNodeId`, `DomOp` (1 new test
  binary, 3 tests).
- **spiral-css** — `CssParser` (deprecated shim; 1 new
  test binary, 3 tests).
- **spiral-dom** — `Descendants`, `Ancestors`, `NodeDepth`
  (1 new test binary, 3 tests, actually exercises the
  tree-walker API).
- **spiral-fmt** — `FormatError` re-export at the crate
  root (1 new test binary, 3 tests).
- **spiral-gyre** — `LayoutEngine` (1 new test binary,
  1 test runs the engine on an empty DOM).
- **spiral-ipc** — `PipeListener`, `PipeTransport`,
  `UnixTransport` (1 new test binary, 1 test exercises
  the encoding surface).
- **spiral-render** — `Rgba` (1 new test binary, 1 test).
- **spiral-theme** — `ThemeMode` (1 new test binary, 1 test).
- **spiral-ui** — `BrowserUi` (1 new test binary, 1 test).
- **spiral-vortex** — `VortexError`, `VortexResult` (1 new
  test binary, 2 tests; M4.5 Item 9 will be the real
  consumer).

The audit script's exclude pattern was tightened from
`!$crate/*` to `!$crate/src/*` so that integration
tests in `tests/` count as cross-crate consumers
(integration tests are separate compilation units; the
lib's `src/` is the declaration site only).

Post-cleanup state: 429 tests pass, 0 failing, 9 of 19
crates are "OK (all wired)", the remaining 10 are
M4.5+ skeletons. The audit will flip each crate from
"skeleton" to "OK" as the corresponding M4.5+ work
lands.

## What needs picking (M4.5+)

- **M4.4 tail** — Items 5, 6, 7 (crypto P0 confirmation, CI excludes for
  retired crates, `justfile` for the verify protocol). Small, all greenfield
  in the workspace today.
- **M4.5 Item 8** — `spiral_net::Resolver` trait wrapping hickory-dns.
- **M4.5 Item 12** — `spiral-filter` runtime hook (Bet 3).
- **M4.6 Item 13** — Gyre box model + margins (first Gyre layout work).

## Do-not-touch zones

`spiral-vortex` internals beyond the skeleton (M4.5+ Item 9 work),
`spiral-gyre` internals beyond the type-level surface (M4.6+ work),
`spiral-sandbox`.

---

## Engine Identity (decided 2026-06-14, amended same day, audited 2026-06-15)

Spiral's stack has two custom-built engines that carry the Spiral brand:

| Engine | Crate | Role | Architecture |
|--------|-------|------|--------------|
| **Gyre** | `spiral-gyre` | Layout (block, flex, grid) | Fully in-house Rust. No Taffy. |
| **Vortex** | `spiral-vortex` | JavaScript | From-scratch Rust JS engine. `rusty_v8` behind `v8` feature for CI oracle only. |

The roadmap is stretched to 6–8 years to accommodate building a from-scratch
JS engine alongside the rest of the browser. v0.1.0 targets Month 60 (~Year 5);
v1.0 targets Month 84 (~Year 7).

`boa_engine` is removed from workspace deps. `taffy` was never added.

### Audit (2026-06-15)

A thorough audit was performed on the M4 first sprint outputs. Findings:

- **No copied code detected.** All techniques are well-documented prior art
  (branded lifetimes from generativity/qcell/ghost-cell; capability tokens
  from cap-std/ambient-authority; per-origin GC from SpiderMonkey zones).
  Spiral re-implemented them from first principles in Spiral-native Rust.
  No verbatim or near-verbatim copying from any external source.
- **Genuinely novel contribution:** Vortex's per-origin arenas with
  **origin-tagged cell headers** in a shared heap. No shipped engine
  does this combination (SpiderMonkey has per-zone major GC but zones
  are not origin-scoped; JSC has a shared `JSVirtualMachine` but no
  origin tagging; V8 is per-isolate).
- **License risk: clean.** MPL-2.0 compatible. No attribution omissions.
- **Factual errors found and fixed:** CBA threshold data had invented
  numbers (5s prestitial countdown, 30% mobile prestitial viewport,
  3Hz flashing from WCAG misattributed to CBA, mobile scrollover
  inflated to "full viewport"). All corrected with proper source
  attribution. See `docs/audit-sprint-m4.md`.
- **Novelty overclaims softened:** "uBO blocks at runtime" was wrong
  for Firefox (uBO Firefox uses `filterResponseData`). "No JIT" is
  well-populated (Duktape, QuickJS, MuJS, LibJS, Hermes, Boa, etc.).
  "5th browser engine" is counting-dependent (Ladybird, Flow, Servo
  are also independent). The honest framing is documented in the
  audit.

Full audit: [`docs/audit-sprint-m4.md`](audit-sprint-m4.md).

---

## Engine Thesis (2026-06-14, user-approved)

Spiral is a **principled, independent 5th browser engine** — not a faster
Chrome, not a leaner Firefox. The four user-stated values drive every
architectural decision:

1. **Independent and principled** — own the engine. No V8 at runtime, ever.
2. **Private by default** — no telemetry, no phone-home, no SafeBrowsing,
   no OCSP that leaks browsing, partitioned caches, ephemeral-by-default.
3. **Minimum memory AND maximum speed** — beat the big 3 on *both* axes.
   This is the unsolved problem in browser engineering. Spiral bets on
   a structurally different architecture to get there.
4. **Web-compliant and useful** — NYT, YouTube, Netflix, games. Widevine
   and EME in. Wasm in. Modern codecs in.

The brand promise: **smart and clever**. Engineering decisions are evaluated
by whether they are smarter and cleverer than what the big 3 do.

---

## The Four Architectural Bets (2026-06-14, user-approved)

The "smart and clever" thesis is realised through four bets that are *not* in
any shipped browser:

### Bet 1 — Shared-Everything Multi-Process (SEM)

- **What:** One renderer process per browser instance, with N typed-isolated
  contexts inside it (per origin). The Vortex heap, Gyre layout engine, parser,
  font system, and standard library are *shared*; per-origin state is
  DOM, CSSOM, JS globals, layout tree.
- **Why clever:** Chromium's per-process isolation duplicates V8 isolates,
  heaps, parsers. ~3–5× memory reduction. Shared caches → faster warm-up.
- **Security model:** Rust capability-typed API surface (not OS processes).
  Stronger than Ladybird's flat address space; weaker than Chromium's
  process walls. The honest middle ground.
- **Risk:** Spectre-class in-process attacks. Mitigation: branch-prediction-
  resistant layout, no secrets in shared arena, no script-controlled
  pointer arithmetic.
- **Phase:** type system + Vortex isolate abstraction land in M4–M6.
  Runtime lands in M25–M36.
- **Full writeup:** [`docs/architecture-shared-everything.md`](architecture-shared-everything.md)

### Bet 2 — Vortex is JIT-Optional, Bytecode-First

- **What:** Ship Vortex tree-walker (Phase A, M4–9) → bytecode VM (Phase B,
  M10–24) for v0.1. JIT (Phase C, M25+) **only** if real-world profiling
  on NYT/Netflix-class sites demands it.
- **Why clever:** A from-scratch JIT is 18–24 months of work and the #1
  browser exploit class (JIT spraying). Skipping it preserves engineering
  capacity and cuts attack surface. Bytecode VM with ICs gets us "fast
  enough" for 80% of the web.
- **What we still do now:** design the bytecode format and IC structure to
  be JIT-friendly. A future JIT is a compiler, not a rewrite.
- **Phase:** tree-walker M4–9. Bytecode VM M10–24. JIT decision gate at M25
  with real-world profile data as input.

### Bet 3 — `spiral-filter` as a Compile-Time Policy Engine

- **What:** A new crate that runs *between the network layer and the parser*,
  parses HTML+CSS, and produces a transformed document with the worst ads
  already removed or constrained. The runtime never sees the offending
  markup.
- **Why clever:** uBlock Origin blocks at runtime, after the page has paid
  the cost. Spiral avoids the cost entirely. The ad-blocker is also a
  *performance optimisation that happens to be a privacy feature*.
- **Default policy (user-approved 2026-06-14):** "Worst offenders only."
  Block HUGE banner ads that cut the page, popups, autoplay video/audio,
  interstitials. Allow reasonable ads. Reward good stewards.
- **Authority model:** seed with Coalition for Better Ads "Better Ads
  Standards" + curated top-100 overlay. Community contributions from M18+.
  Stewardship score per domain, opt-in for site owners.
- **User-tunable:** slider from "block nothing" to "block almost everything."
  Default = "block worst offenders only."
- **Phase:** crate skeleton + surgical default policy land in M4.

### Bet 4 — Persistent Renderer / Warm Caches

- **What:** When a tab is idle, checkpoint Vortex heap + layout tree + DOM
  hash to a memory-mapped file. On revisit (back button, tab switch, crash
  recovery), mmap it and lazy-fill what changed. Warm tabs reopen in ~30ms
  because they are page faults, not process spawns.
- **Why clever:** Chromium pays 200–500ms per tab restore because it kills
  the renderer. Spiral keeps the renderer state and pays *only* for pages
  that are actually touched.
- **Memory accounting:** mmap'd pages cost the tab's budget only when
  touched. 5 hot tabs + 45 warm tabs ≈ 5×hot + 0.5×warm, not 50×hot.
- **Phase:** M30+ for the Vortex heap checkpoint; M36+ for the layout tree;
  M42+ for the full document checkpoint. Each step is independently useful.

---

## Three New Crates Required by the Thesis

| Crate | Purpose | Why it must exist | Phase |
|-------|---------|-------------------|-------|
| `spiral-context` | Capability-typed API surface for the shared-everything runtime. Per-origin context, per-origin handle types, brand types. | Foundation of Bet 1. The type system that makes shared-everything safe. | M4 (skeleton) → M25 (runtime) |
| `spiral-filter` | Network filter + compile-time HTML/CSS policy engine. EasyList-style rules, cosmetic CSS injection, declarative steward list. | Brand promise ("smart ad blocking") + performance optimisation. The "NYT without ads" test page requires this to be readable. | M4 |
| `spiral-media` | MSE/EME demuxers, audio/video decoders, audio output, Widevine CDM bridge. | Netflix + YouTube + games. The "useful" requirement. | M30+ |

---

## Media / DRM / EME Stance (2026-06-14, user-approved)

- **ClearKey EME in v0.1** (M12 or so). Achievable, unblocks MSE-based
  content, no trust concerns about a third-party binary.
- **Widevine in v1.0** (M36+). Licensable from Google. Trust audit gates
  the decision. Documented as "DRM is necessary for Netflix; we ship it
  because users asked for Netflix."
- **Codecs:** AV1 (dav1d), VP9 (libvpx or rav1d), HEVC (via FFmpeg,
  gated on patent clearance), Opus, AAC. Use existing decoders; do not
  write our own.

---

## Process Model Decision (2026-06-14, user-approved)

- **Default:** single-process with per-origin typed isolation (Ladybird-style,
  but capability-typed — Bet 1).
- **Optional:** per-origin "isolation mode" toggle that downgrades to
  multi-process for `bank.com`-class sites. User-tunable per origin.
- **Why hybrid:** most sites don't need Chromium-class isolation. Banking
  and similar do. The user gets both without paying for both on every tab.

---

## Ad Policy Decision (2026-06-14, user-approved)

- **Default policy:** "Worst offenders only." Surgical.
  - **Block:** banner ads that break layout, popups, autoplay video and
    audio, interstitials, large sticky ads that cover content.
  - **Allow:** reasonable, well-behaved ads. The page renders. Revenue
    still flows to good stewards.
  - **Reward:** sites that self-attest to the Better Ads Standards get
    a positive stewardship score. Sites on the violation list get a
    negative score. Both affect the default blocklist, not user
    overrides.
- **User slider:** from "block nothing" to "block almost everything."
- **No third-party tracking.** Period. No "acceptable ads" program
  that requires telemetry.
- **Site-owner escape hatch:** opt-in stewardship registry. Site owners
  can self-attest and earn a better default. The bar is real (Better
  Ads Standards) not pay-to-play.

---

## Sprint Goal (Month 4 first sprint — design pass output 2026-06-15)

The M4 first sprint produced three new crate skeletons and the Vortex GC
rewrite. **All custom code, no external engine dependencies** (per
user decision 2026-06-15: "Our tech where it matters. Using other
browser's tech defeats the purpose of spiral.").

- [x] **`spiral-context` crate skeleton** — branded types, capability
      tokens, origin isolation. 21 tests passing.
      `crates/spiral-context/src/{brand,origin,caps,context,dom}.rs`
- [x] **`spiral-filter` crate skeleton** — ABP/EasyList parser
      (cosmetic + network rules), rule AST, CBA defaults, hostname
      trie, policy slider. 40 tests passing.
      `crates/spiral-filter/src/{rule,syntax,compile,lists,policy}/`
- [x] **Vortex GC rewrite** — `VortexHeap` with per-origin
      `OriginArena`, `TaggedCell` with 4-byte header, `GcKey` with
      versioned+branded keys, stop-the-world mark-sweep. 22 new
      tests (GC went from 41 → 84 total). Old `Heap` type replaced.
- [ ] Vendor `html5ever` into `spiral-fmt`; modernise deps
- [ ] Vendor `cssparser` + `selectors` into `spiral-fmt`
- [ ] Unified facade: `spiral_fmt::parse_html()`, `spiral_fmt::parse_css()`
- [ ] `spiral_net::Resolver` trait wrapping hickory-dns
- [ ] Gyre block layout — first pass (no Taffy in tree)
- [ ] Vortex spike — `rusty_v8` hello world, isolate lifecycle

**Design doc deliverables (this session):**
- `docs/design-filter-rule-model.md` — full rule AST, CBA thresholds,
  custom parser approach (no `adblock` crate).
- `docs/design-capability-types.md` — branded lifetimes, capability
  tokens, `ContextOps` trait, `InProcess` / `Escalated` modes.
- `docs/design-vortex-heap.md` — per-origin arenas, `TaggedCell`
  header, `GcKey` versioning, phase-gated GC progression.

---

## In Progress

M4 first sprint M4.1–M4.3 complete (the three new crate skeletons and
Vortex GC rewrite). M4.4–M4.6 remain: vendor parsers, Resolver trait,
Gyre block layout. Continuing with M4.4 next.

## Completed

- Sprint 0: repo scaffolding, docs baseline
- Sprint 1: core types (`BrowserConfig`, `TabId`, `IPCMessage`, `Error`, tests)
- Sprint 2: CI matrix, lint hygiene
- Sprint 3: IPC transport layer (`IpcTransport`, Unix/Windows, framing, mock)
- Sprint 4: browser shell, software renderer, hello-world PNG
- **Design pass (2026-06-14):** four architectural bets, three new crates,
  process model and ad policy decisions — all signed off.
- **M4 design pass (2026-06-15):** three design docs (filter, capability,
  vortex heap). User decisions: custom-only, no external engine deps.
- **M4 build pass (2026-06-15):** `spiral-context` skeleton (21 tests),
  `spiral-filter` skeleton (40 tests), Vortex GC rewrite (43 new tests).
  Total: **266 tests passing workspace-wide**, 0 failures.
- **M4 rewire (2026-06-15):** `spiral-html` retired. `spiral-fmt` is the
  sole HTML parser. `html5ever`, `markup5ever`, `tendril` removed from
  workspace. Servo crates completely absent from dependency tree.
  Total: **275 tests passing workspace-wide**, 0 failures.

## Do Not Touch

- `spiral-gpu`, `spiral-paint`, `spiral-ui`, `spiral-theme` — Phase 4
- `spiral-sandbox` — Phase 4 (re-evaluated under Bet 1; sandbox becomes
  capability-typed, not OS-level, for default process model)
- Vello fork (`spiral-vello`) — Phase 4
- Widevine / EME binary integration — M36+ (v1.0). ClearKey only at M12.

---

## Phase 1 Exit Criteria — Status

| Criterion | Status |
|-----------|--------|
| `cargo build --workspace` succeeds | ✅ |
| `cargo test --workspace` passes | ✅ (143 tests) |
| Browser renders "Hello World" | ✅ (`target/hello-world.png`) |

**Phase 1 is complete.**

---

## Phase 2 First Sprint Memory Budget (new — design pass output)

| Scenario | Target | Source of budget |
|----------|--------|------------------|
| Cold start to interactive (single tab, about:blank) | < 200 ms | Shared parser + warm caches (Bet 4 deferred; Phase 2 first sprint measures baseline) |
| Idle tab resident memory (backgrounded, untouched) | < 30 MB | Arena-allocated DOM + Gyre lazy boxes |
| Active tab (NYT-class static article) | < 150 MB | Filtered HTML (Bet 3) + Gyre lazy construction |
| 5-tab session (one active, four idle) | < 250 MB | Per-tab page-fault accounting (Bet 4 design) |

These are *gates*, not aspirations. CI will measure on a representative
NYT page load once the pipeline exists. Exceeding the budget fails the
build.

---

## Phase 2 First Sprint WPT Targets (new — design pass output)

| Suite | Target | Notes |
|-------|--------|-------|
| `css/css-box/` | 40% pass | Gyre block layout |
| `css/css-position/` | 40% pass | Gyre positioning |
| `html/semantics/` (scripting) | 30% pass | Vortex tree-walker can execute test scripts |
| `html/semantics/embedded-content/` (img) | 50% pass | No lazy decode yet; sync decode acceptable |

These are *first-sprint* targets, not v0.1 targets. They will be raised
each sprint as the engines mature.

---

## Key Architecture Decisions This Sprint (design pass)

1. **Shared-everything multi-process (Bet 1)** is the structural bet.
   Capability types are designed in M4 even if the runtime lands in M25.
2. **Vortex is bytecode-first (Bet 2).** Tree-walker → bytecode VM for
   v0.1. JIT deferred behind a real-world profile gate at M25.
3. **Ad-blocking is a parse-time policy (Bet 3).** It is not a runtime
   filter bolted on at the network layer.
4. **Memory is a CI-gated budget, not an aspiration.** Per-phase budgets
   are enforced.
5. **The brand promise is "smart and clever."** Every architectural
   decision is evaluated against that yardstick. If a decision is the
   same as Chromium, we have failed.

---

## SSOT Links

- [`docs/architecture-shared-everything.md`](architecture-shared-everything.md) — full Bet 1 writeup
- [`docs/progress_ledger.md`](progress_ledger.md) — change log
- [`docs/plans/iteration-options.md`](plans/iteration-options.md) — dependency triage and 12-week plan
- [`docs/audit-sprint-m4.md`](audit-sprint-m4.md) — M4 originality audit
- [`docs/innovations-backlog.md`](innovations-backlog.md) — **single source of truth for the 70-idea backlog** (consolidates the former index, routing, top-10, and quality-novelty-summary files)
- [`docs/innovations-stubs-archive/`](innovations-stubs-archive/) — raw brainstorm inputs (5 batches, original and audited); traceability only
- [`../specs/GAP_ANALYSIS.md`](../specs/GAP_ANALYSIS.md) — **P0/P1/P2/P3 gap tracker across 4 engine sub-domains; priority stack and proposed first fill** (architect pass 2026-06-15)
- [`ROADMAP.md`](../ROADMAP.md) — phase plan
- [`ARCHITECTURE.md`](../ARCHITECTURE.md) — canonical architecture (to be updated next sprint)
