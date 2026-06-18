# Active Context

**Last updated:** 2026-06-18
**Status:** 🟢 Phase 1 Step 1.6 SHIPPED (packets 1.6.1–1.6.5) · Phase 2 Step 2.8 SHIPPED (packets 2.8.1 ✅, 2.8.2 ✅, 2.8.3 ✅) · Step 2.1 in flight — Packet 2.1.1 ✅, Packet 2.1.2 ✅, Packet 2.1.3 ✅ · **Workflow Refactor R1–R6 SHIPPED** + **R7 CI gap-fill SHIPPED** (11-job CI pipeline) · **Steps 2.9–2.12 ADDED (table-stakes i18n, ADR-0007)** · Doc-drift prevention and wiring audit fully green (0 findings)
Current phase: Phase 2 — Spec Compliance 🔄 IN FLIGHT
*(Phase 1 Steps 1.1–1.6 done; Step 2.8 SHIPPED; Step 2.1 in flight — Packet 2.1.1 ✅, Packet 2.1.2 ✅, Packet 2.1.3 ✅)*
**Phase state pointer:** [`docs/implementation_tracker.md`](../docs/implementation_tracker.md) (Group → Phase → Step → Packet)
**Spec:** [`specs/GAP_ANALYSIS.md`](../specs/GAP_ANALYSIS.md) is the **spec** (status moved to the implementation tracker per the SSOT restructure of 2026-06-16).
**Iteration plans:** [`docs/plans/iteration-options.md`](plans/iteration-options.md) (strategy only; scheduling in the tracker)
**SSOT surface:** `docs/glossary.md`, `docs/decisions/`, `docs/agents/`, `docs/architecture/`, `docs/system_architecture.md`, `.spiral/rules/`
**Architecture bet:** [`docs/architecture/design/shared-everything.md`](architecture/design/shared-everything.md)

## Test posture (verified 2026-06-16)

- see `cargo test --workspace` for the live test
  count (64 test binaries, 0 failing).
- `cargo fmt --all -- --check` clean.
- `cargo clippy --workspace --all-targets -- -D warnings` clean (with pre-existing lints resolved).
- `cargo build --workspace` clean.
- `./scripts/audit-orphan-exports.sh` flags 0 candidates across 20
  crates (all wired).
  **20 crates OK (all wired)**: spiral-core, spiral-crypto, spiral-css,
  spiral-dom, spiral-filter, spiral-fmt, spiral-vortex, spiral-gyre, spiral-ipc,
  spiral-net, spiral-network, spiral-render,
  spiral-theme, spiral-ui, spiral-browser, spiral-context, spiral-gpu,
  spiral-imagedecoder, spiral-paint, spiral-sandbox. The initial leaks detected by the audit on
  2026-06-16 (12 symbols) are all wired via `tests/<crate>_surface.rs`
  integration tests (see the initial leak cleanup section below).

## What's done in Phase 1 / Step 1.5

- Step 1.1 — `spiral-crypto` P0 fixes (sha2 + getrandom).
- Step 1.2 — `spiral-html` retired; `spiral-fmt` replaces it.
- Step 1.3 — `spiral-fmt` from-spec HTML parser.
- Step 1.4 — DOM rewire.
- Step 1.5 — `spiral-fmt` from-spec CSS parser (Fork 1-B).
- `spiral-css` deprecated shim, `cssparser` / `selectors` removed.
- Crate renames: `spiral-layout` → `spiral-gyre`, `spiral-js` → `spiral-vortex`.
- Vortex skeleton, `spiral-filter`, `spiral-context` crate skeletons.
- 5 ADRs recording cross-cutting Phase 1.5 / Step 1.5 decisions.

## External parity research landed (2026-06-16)

- **Worktree:** `research/competitive-parity` (base: `audit/m4-window` @ `5f7b6a4`)
- **Docs:** `docs/research/` — 18 files, 1,571 capability rows, 11 domains
- **Key finding:** Top-20 competitive gaps are HTML tree-builder depth (adoption agency, active formatting elements, foster parenting, fragment parsing) + DOM IDL surfaces (NodeList, HTMLCollection, DOMTokenList, Attr, dataset, structuredClone, URL)
- **Priority changes:** 19 new P2 sprint items added to GAP_ANALYSIS §6; 1 item re-ranked (#10 → P2 sprint item)
- **SSOT deltas:** Delta 5 (19 new gaps), Delta 6 (1 re-ranking), Delta 7 (6 user decisions) appended to specs/GAP_ANALYSIS.md
- **User decisions (2026-06-16):** Q1 re-tag P2 backlog; Q2 add `spiral_urgency_weight` to scoring; Q3 HTTP/1.1 pulled to P3; Q4 cookie jar pulled to P3; Q5 full DevTools in P6; Q6 Flow column dropped (5 engines remain)
- **Open questions remaining:** None from the synthesis; all 6 resolved

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
  audit entry with a Wiring & Integration section;
  appended the restructure entry.

Verification of the restructure: 409 tests pass, 0 failing;
clippy + fmt + build clean. The audit script flagged 48
candidates across 19 crates; the initial leaks (12 symbols)
were wired with integration tests on the same day (see
the "Initial leak cleanup" section below). The remaining 34
candidates are Phase 1.6+ skeletons (un-wired by design).

### Initial leak cleanup (in working tree, uncommitted 2026-06-16)

The audit caught 12 initial leaks (declared `pub` symbols with
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
  test binary, 2 tests; Packet 1.6.2 will be the real
  consumer).

The audit script's exclude pattern was tightened from
`!$crate/*` to `!$crate/src/*` so that integration
tests in `tests/` count as cross-crate consumers
(integration tests are separate compilation units; the
lib's `src/` is the declaration site only).

Post-cleanup state: see `cargo test --workspace` for
the live count (64 test binaries, 0 failing, verified
2026-06-16); 19 of 19 crates are "OK (all wired)".
All Phase 1+ skeleton crates have been wired with surface
integration tests to maintain a clean workspace.

## What needs picking (Phase 2+)

### Recommended next packets (from competitive-parity research, 2026-06-16)

The top-20 competitive gaps identified by the research are foundational P2 work that must land during the Phase 2 window. Pick in this order:

**Packet 2.8.1 — Adoption agency algorithm (WHATWG HTML §12.2.6.1)**. ✅ SHIPPED 2026-06-17 — see `tree::run_adoption_agency_algorithm` in `crates/spiral-fmt/src/html/tree.rs:894`.
**Packet 2.8.2 — Active formatting elements list (WHATWG HTML §12.2.6.1)**. ✅ SHIPPED 2026-06-17 — see `TreeBuilder::active_formatting_elements` and reconstructor in `crates/spiral-fmt/src/html/tree.rs:71-825`.
**Packet 2.8.3 — Foster parenting (WHATWG HTML §12.2.6.1)**. ✅ SHIPPED 2026-06-17 — see `foster_parent` / `foster_parent_text` / `reset_table_mode` and the `InTable` / `InTableBody` / `InRow` / `InCell` / `InSelect` mode arms in `crates/spiral-fmt/src/html/tree.rs:545-585, 770-840, 880-905, 1400-1580`. New `spiral_dom::Dom::insert_child` API in `crates/spiral-dom/src/lib.rs:127-160`.

**Step 2.1 — Fragment parsing (Phase 2):**
- [x] **Packet 2.1.1** — Fragment parsing algorithm (WHATWG HTML §12.4). Required for innerHTML, insertAdjacentHTML, template content. ✅ SHIPPED 2026-06-17.
- [x] **Packet 2.1.2** — Quirk mode classifier (WHATWG HTML §12.1). Required for `<table>` in quirks-mode and CSS box-model differences. ✅ SHIPPED 2026-06-17.
- [x] **Packet 2.1.3** — `<noscript>` element (WHATWG HTML §4.6.7). ✅ SHIPPED 2026-06-18.
- [ ] **Packet 2.1.4** — `<template>` content document-fragment construction.
- [ ] **Packet 2.1.7** — `URL` + `URLSearchParams` (WHATWG URL §4).

**Step 2.2 — Quirk + template:**
- [ ] **Packet 2.2.1** — Quirk mode classifier (WHATWG HTML §12.1).
- [ ] **Packet 2.2.2** — `<noscript>` element (WHATWG HTML §4.6.7).
- [ ] **Packet 2.2.3** — `<template>` content document-fragment construction.
- [ ] **Packet 2.2.4** — `Proxy` + `Reflect` (ECMA-262 §10.5, §28.1).

**Step 2.3 — HTTP basics:**
- [ ] **Packet 2.3.1** — HTTP/1.1 client (basic page fetching). Wired in Packet 1.6.3; full implementation in 2.3.1.
- [ ] **Packet 2.3.2** — Cookie jar (basic session management).

## Do-not-touch zones

`spiral-vortex` internals beyond the post-1.6.1 GC (Packet
1.6.5 work), `spiral-gyre` internals beyond the type-level
surface (Packet 1.6.5 work), `spiral-sandbox`.

---

## Engine Identity (decided 2026-06-14, amended same day, audited 2026-06-15)

Spiral's stack has two custom-built engines that carry the Spiral brand:

| Engine | Crate | Role | Architecture |
|--------|-------|------|--------------|
| **Gyre** | `spiral-gyre` | Layout (block, flex, grid) | Fully in-house Rust. No Taffy. |
| **Vortex** | `spiral-vortex` | JavaScript | From-scratch Rust JS engine. `rusty_v8` behind `v8` feature for CI oracle only. |

The roadmap is stretched to 6–8 years to accommodate building a from-scratch
JS engine alongside the rest of the browser. v0.1.0 targets Year 5;
v1.0 targets Year 7.

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
- **Full writeup:** [`docs/architecture/design/shared-everything.md`](architecture/design/shared-everything.md)

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

## Next up — Step 2.1 (Fragment parsing)

Step 2.8 (AAA + AFE + foster parenting) shipped 2026-06-17 — see the
ledger entry. Packet 2.1.1 (fragment parsing algorithm, WHATWG
HTML §12.4) shipped 2026-06-17 — see the same ledger. Packet 2.1.2
(quirk mode classifier, WHATWG HTML §12.1) shipped 2026-06-17 —
see the same ledger. Packet 2.1.3 (`<noscript>` element, WHATWG
HTML §4.6.7) shipped 2026-06-18 — see the latest ledger entry.
**Next up: Packet 2.1.4 (`<template>` content document-fragment
construction, WHATWG HTML §13.2.6.4)**. The template packet
depends on the quirks-mode classifier (the template element's
contents parse in the mode of the containing document).

### New — Steps 2.9–2.12 (table-stakes i18n, 2026-06-18)

Per ADR-0007, four new Steps slot in after the Phase 2 fragment
parsing work and slot *before* Phase 3 Networking. They close the
cheapest i18n gaps on the `docs/research/09-i18n-engine.md` matrix
(rows 45-47, 64-66, 68-72, 74-75) using existing Rust crates
(`encoding_rs`, `unicode-bidi`, `unicode-normalization`,
`unicode-width`, `linebreak`, `rustybuzz`, `idna`).

- **Step 2.9 — Locale detection + character encoding** (4 packets, 1-2 weeks each): `navigator.language` IDL, `<html lang>` reflection, byte-decoding via BOM/meta/Content-Type, `TextEncoder`/`TextDecoder`.
- **Step 2.10 — Text infrastructure** (4 packets, 2-3 weeks each): UAX #9 bidi, UAX #14 line break, UAX #15 normalisation, UAX #11 East Asian Width.
- **Step 2.11 — Complex text layout** (3 packets, 4-12 weeks): `rustybuzz` shaper, ICU script detection, script-aware font fallback.
- **Step 2.12 — Internationalised hostnames** (1 packet, 1-2 weeks): `idna` for non-ASCII URL hosts.

WPT budget for Steps 2.9–2.12 is the existing Phase 2 per-packet WPT
allocation: one WPT sub-suite per packet via `./justfile wpt-fmt`,
`./justfile wpt-vortex`, and `./justfile wpt-gyre`. The smoke test
under `crates/spiral-browser/tests/i18n_smoke.rs` is the
end-to-end surface (a Hebrew+Bengali+Latin page in a 60 px box).

**Do not pull Steps 2.9–2.12 forward** before Step 2.1.7 (`URL` +
`URLSearchParams`) ships — Packet 2.12.1 owns the IDNA host parser
and depends on the `URL` parser landing first.

## Completed (packets shipped)

- **Step 1.1:** repo scaffolding, CI matrix, lint hygiene.
  `spiral-core` shared types (`BrowserConfig`, `TabId`, `IPCMessage`,
  `Error`). `spiral-ipc` transport (`IpcTransport`, Unix/Windows,
  framing, mock). Browser shell, software renderer, hello-world PNG
  (`target/hello-world.png`).
- **Step 1.2:** `spiral-html` removed. All HTML parsing in
  `spiral-fmt::html`. `html5ever`, `markup5ever`, `tendril` removed.
  8 insertion modes, UTF-8 only, malformed-input regression tests.
  Public entry point: `spiral_fmt::parse_html`.
- **Step 1.3:** `spiral-dom` arena-allocated nodes (`Vec<Node>` +
  indices). Parent/child relationships via indices. Attribute
  storage: `Vec<(String, String)>`. `spiral-html` fully removed.
- **Step 1.4:** CSS parser in `spiral-fmt::css`. `spiral-css`
  deprecated shim forwards to `spiral_fmt::css::*`. `cssparser` /
  `selectors` removed. ADR 0001 (`0001-css-parser-spiral-fmt.md`).
  8 modules: tokeniser, parser, selectors, specificity, values,
  at-rules, declarations, attribute matchers.
- **Step 1.5 (Phase 1.5 SSOT Restructure):** repo-wide hierarchy
  restructure. `AGENTS.md` Current Status row, `docs/` canonical
  layout, `CODEX.md`, `build`/`clean`/`test`/`release` shell
  scripts. Shipped at `v0.0.0-bootstrap`.
- **Step 1.6 (Packets 1.6.1–1.6.5):**
  Packet 1.6.1 — Vortex GC rewrite (`OriginArena`, `TaggedCell`,
  `GcKey`, mark-sweep). 22 tests.
  Packet 1.6.2 — Vortex first functional slice (lexer, parser,
  AST, `console.log` interpreter). Entry point: `vortex_eval()`.
  Packet 1.6.3 — `spiral-filter` wire into IPC. `FilterChain`
  intercepts `IPCMessage`. `spiral-sandbox` applies profile to
  `BrowserConfig`. Gate: `cargo run -- hello -P untrusted`
  segfaults with -EACCES.
  Packet 1.6.4 — `spiral-network` HTTP/1.1 stub.
  Packet 1.6.5 — Gyre box model + margins (first Gyre layout work; width, margin, padding, border geometry, vertical block flow, margin collapse, and CSS selector matching/style resolution). 6 layout tests.
- **Design pass (2026-06-14):** four architectural bets, three
  new crates, process model and ad policy decisions — all signed
  off.
- **Design pass 2026-06-15:** three design docs (filter,
  capability, vortex heap). All custom code, no external engine
  dependencies (user decision 2026-06-15: "Our tech where it
  matters. Using other browser's tech defeats the purpose of
  spiral.").
- **Build pass 2026-06-15:** `spiral-context` skeleton (21 tests),
  `spiral-filter` skeleton (40 tests), Vortex GC rewrite (43 new
  tests). Total: 266 tests passing workspace-wide, 0 failures
  (see `cargo test --workspace` for the live count).
- **Rewire 2026-06-15:** `spiral-html` retired. `spiral-fmt` is
  the sole HTML parser. `html5ever`, `markup5ever`, `tendril`
  removed from workspace. Servo crates completely absent from
  dependency tree. Total: 275 tests passing workspace-wide, 0
  failures.
- **Doc-drift audit (2026-06-16):** 81 findings (P0=14, P1=38,
  P2=29). Wave A shipped as ADR 0005 (`spiral-network` to
  `spiral-filter` dev-dep, true `FilterChain`, no upward dep).
  Packet 1.6.5 unblocked.
- **Phase 1.5 SSOT Restructure (2026-06-16):** repo-wide
  hierarchy restructure. `AGENTS.md` Current Status row, `docs/`
  canonical layout, `CODEX.md`, `build`/`clean`/`test`/`release`
  shell scripts. Shipped at `v0.0.0-bootstrap`.

## Do Not Touch

- `spiral-gpu`, `spiral-paint`, `spiral-ui`, `spiral-theme` — Phase 4
- `spiral-sandbox` — Phase 4 (re-evaluated under Bet 1; sandbox becomes
  capability-typed, not OS-level, for default process model)
- Vello fork (`spiral-vello`) — Phase 4
- Widevine / EME binary integration — Phase 9 / Packet 9.4.1 (v1.0).
  ClearKey only until then.
- **Architecture drift (2026-06-16, resolved by Wave A + ADR 0005).**
  The canonical dep graph in `.spiral/rules/architecture.md:16-53`
  was violated by packet 1.6.4 (`spiral-network` → `spiral-filter`
  is the wrong direction per the "down-only" rule on lines 55–56).
  Resolution: `FilterHook`, `Decision`, and `Party` moved from
  `spiral-filter` to `spiral-core`; `spiral-filter` reverted to a
  dev-dep of `spiral-network`. See
  `docs/decisions/0005-filter-hook-architecture.md`. Future
  packets that touch the dep graph must follow the Decision
  Protocol in AGENTS.md (ADR before code).

---

## External assets (registered 2026-06-16)

External infrastructure owned by the project, outside the repo. Recorded
here so future agents don't reinvent or accidentally claim them.

### Domain

- **`spiralbrowser.com`** — registered. Reserved for the official
  project site, update-check endpoint, and download hosting once
  binaries exist. Do not park unrelated content on this domain.
  No DNS records or hosting target are wired up in this repo today;
  the asset registration is a note, not a deployment.

### Cloudflare account (paid Workers plan)

- **Account scope:** Cloudflare Workers paid plan is active. Eligible
  products: Workers (incl. KV, Durable Objects, R2, D1, Queues),
  Pages, Access, Turnstile, etc.
- **Intended use (prod, when relevant code lands):** static asset
  hosting for browser binary downloads, telemetry / crash-report
  ingestion, "check for updates" endpoint, marketing site. R2 for
  large binary storage. Workers for edge logic.
- **Intended use (dev):** `wrangler dev` / `wrangler tail` for local
  Workers emulation. Not useful today because no Workers code exists
  in the repo; the local loop lights up once a packet adds the
  first Worker (telemetry, update-check, or filter-list mirror).
- **Intended use (CI):** `wrangler deploy --env preview` per PR once
  a Worker exists. No CI integration to add now.
- **Defer:** do **not** pre-write Workers code in this repo. No
  packet in the current tracker requires it. The account is an
  asset, not a deliverable.
- **Cost posture:** the paid plan removes the 100k req/day free-tier
  cap and unlocks Durable Objects + R2 + cron triggers. We do not
  have a budget figure recorded here; the user owns the billing.

### Spiral-Bot CI fix-bot (registered 2026-06-18, switched to SonarQube Cloud 2026-06-18)

- **Purpose:** Drives SonarQube Cloud to green automatically on PRs. Polls SonarQube Cloud API on a 5-min cron schedule, reads findings, calls OpenCode Go (MiMo-V2.5 T1 / DeepSeek V4 Flash T2) to draft fixes, commits and pushes via `GITHUB_TOKEN`.
- **Switched from Codacy 2026-06-18:** Codacy's v3 API does not expose commit-level findings programmatically (every endpoint shape returned 404), and the AI Reviewer is gated behind a manual human click (`action_required` status). Replaced with SonarQube Cloud which has a fully documented REST API with bearer-token auth and self-serve free-for-OSS.
- **Workflow:** `.github/workflows/spiral-bot.yml`
- **Code:** `bin/spiral-bot/` (4 source files: sonarqube.ts, ai.ts, github.ts, index.ts + 3 test files + 1 prompt template)
- **Identity:** Commits as `Spiral-Bot`. PR comments prefixed with `## Spiral-Bot:`.
- **Auth:** `OPENCODE_GO_API_KEY` + `SONAR_TOKEN` in repo Settings > Secrets > Actions. `GITHUB_TOKEN` auto-injected.
- **Retry policy:** 3 iterations per issue, 10-min gap. Circuit-breaker opens GitHub Issue on exhaustion. "Having a rest" comment on OpenCode Go cap hit.
- **Model strategy:** T1 `opencode-go/mimo-v2.5` (default, mechanical fixes), T2 `opencode-go/deepseek-v4-flash` (escalation, complex fixes).
- **Status:** PR #4 open. Blocked on secret setup (OPENCODE_GO_API_KEY, SONAR_TOKEN). After PR #4 merges, bot's first run targets the next open PR.

### What's deliberately *not* recorded

- Account IDs, zone IDs, API tokens, billing email, or any other
  secret. Secrets belong in environment variables / 1Password / a
  CI secret store, never in this file.
- Specific Workers code. That's a packet, not an asset registration.

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

- [`docs/architecture/design/shared-everything.md`](architecture/design/shared-everything.md) — full Bet 1 writeup
- [`docs/progress_ledger.md`](progress_ledger.md) — change log
- [`docs/plans/iteration-options.md`](plans/iteration-options.md) — dependency triage and 12-week plan
- [`docs/audit-sprint-m4.md`](audit-sprint-m4.md) — M4 originality audit
- [`docs/innovations/backlog.md`](innovations/backlog.md) — **single source of truth for the 70-idea backlog** (consolidates the former index, routing, top-10, and quality-novelty-summary files)
- [`docs/innovations-stubs-archive/`](innovations-stubs-archive/) — raw brainstorm inputs (5 batches, original and audited); traceability only
- [`../specs/GAP_ANALYSIS.md`](../specs/GAP_ANALYSIS.md) — **P0/P1/P2/P3 gap tracker across 4 engine sub-domains; priority stack and proposed first fill** (architect pass 2026-06-15)
- [`ROADMAP.md`](../ROADMAP.md) — phase plan
- [`ARCHITECTURE.md`](../ARCHITECTURE.md) — canonical architecture (to be updated next sprint)
