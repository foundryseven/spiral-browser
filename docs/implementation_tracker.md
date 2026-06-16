# Implementation Tracker

Granular task checklist. **Single source of truth for what is built, what is
in flight, and what is missing.** Check off as completed. Do not rewrite
history; append new phases.

**Reading order for a new agent:**

1. Top of this file — Groups and Phases overview.
2. The active Phase's "In flight" section.
3. The "What needs picking" section at the bottom for next-up packets.
4. The linked ADRs and architecture docs as you go.

**Status legend:** `[x]` shipped · `[~]` partial / stub / wired-but-inert · `[ ]` not started.

---

## ⚠️ Wiring & Integration Rule

Every Phase MUST end with a `### Wiring & Integration` subsection. A `pub`
symbol is not done when it compiles; it is done when at least one consumer
outside the symbol's home crate imports it. A function is not done when it
has unit tests; it is done when at least one call site in another crate, or
a public entry point in the same crate's binary surface, exercises it. A
new crate is not done when it has a `Cargo.toml`; it is done when the
binary surface (`spiral-browser` for the end-user app) actually imports
it. The audit script (`./scripts/audit-orphan-exports.sh`) is the ground
truth; exit 1 = build break.

Adopted 2026-06-16 from the Zeus repo's
`docs/decisions/0006-cross-cutting-features.md`. See
`docs/decisions/0000-template.md` for the ADR structure; the "Wiring &
Integration" section is required.

---

## Active Wiring Gaps (orphan exports)

> Audit runs in CI on every push. The most recent local run was 2026-06-16
> against `crates/spiral-*/src/`. Re-run with
> `./scripts/audit-orphan-exports.sh`. Exits 1 on orphans.
>
> Excludes test consumers, intra-lib usage, and type re-exports within
> the same file. The 23 candidates flagged on 2026-06-16 across 6 crates
> are Phase 1+ skeletons (un-wired by design). Each one maps to a packet
> in **Phase 1.6 — Phase 1 wrap-up** (in flight) or a later Phase. The
> audit will flip each crate from "skeleton" to "OK (all wired)" as the
> corresponding packet lands. Packets 1.6.1, 1.6.3, and 1.6.4 already
> closed all orphans in `spiral-vortex`, `spiral-net`, `spiral-network`,
> and `spiral-filter` (down from 34 → 23 orphans across 10 → 6 crates).

---

## Groups

A Group is a capability area of the browser. Phases sit underneath
Groups. The four Groups correspond to the four architectural bets and
the cross-cutting work that does not fit any one subsystem.

| Group | Owned by | Anchor doc |
|-------|----------|------------|
| **Engines** | Vortex, Gyre, Fmt, Filter, Context | [`docs/architecture/fmt.md`](architecture/fmt.md), [`vortex.md`](architecture/vortex.md), [`gyre.md`](architecture/gyre.md), [`filter.md`](architecture/filter.md), [`context.md`](architecture/context.md) |
| **Networking** | spiral-network, spiral-crypto, spiral-ipc, spiral-sandbox | [`docs/architecture/net.md`](architecture/net.md) |
| **Presentation** | spiral-render, spiral-ui, spiral-theme | `docs/architecture/render.md` (TBD) |
| **Cross-cutting** | spiral-core, spiral-browser (binary), `docs/decisions/`, `docs/innovations/` | [`docs/system_architecture.md`](system_architecture.md) |

---

## Phases

A Phase is a major delivery milestone. One Phase = one shipped
user-facing capability. Phases are **not** date-bound. The 0.5 slots are
reserved for cross-cutting restructures (this file is the first one).

| # | Title | Status |
|---|-------|--------|
| **0** | Foundation (IPC shell, hello-world render, crate workspace) | ✅ COMPLETE |
| **1** | Engines Foundation (Vortex slice, CSS/HTML from-spec, Gyre box model, filter runtime) | 🔄 IN FLIGHT |
| **1.5** | SSOT Restructure (this restructure) | 🔄 IN FLIGHT |
| **2** | Engines Depth (top-20 competitive gaps, fragment parsing, DOM collections, dataset, structuredClone, URL) | ☐ NOT STARTED |
| **3** | Networking (HTTP/1.1 client, cookie jar, DNS resolver, sandbox profile, IPC transport hardening) | ☐ NOT STARTED |
| **4** | Presentation (GPU render, chrome UI, theme system, Vello integration) | ☐ NOT STARTED |
| **5** | Capability Types Runtime (Bet 1 type system runtime; per-origin isolate; capability tokens in production paths) | ☐ NOT STARTED |
| **6** | Bytecode VM (Vortex tree-walker → bytecode VM, ICs, real-world profile gate) | ☐ NOT STARTED |
| **7** | Media + DRM (MSE/EME demuxers, ClearKey, Widevine) | ☐ NOT STARTED |
| **8** | Persistent Renderer (Bet 4 — Vortex heap checkpoint, layout tree checkpoint, document checkpoint) | ☐ NOT STARTED |
| **9** | Hardening (memory budget CI gate, WPT coverage, fuzz harnesses, supply-chain review) | ☐ NOT STARTED |

---

## Phase 0 — Foundation ✅ COMPLETE

The IPC shell, hello-world render, and the 19-crate workspace. Phase 0
established the `Spiral = shared-everything multi-process + custom
engines` thesis (see [`docs/system_architecture.md`](system_architecture.md)).

- [x] Repo scaffolding, CI matrix, lint hygiene
- [x] `spiral-core` — shared types (`BrowserConfig`, `TabId`, `IPCMessage`, `Error`)
- [x] `spiral-ipc` — `IpcTransport`, Unix/Windows, framing, mock
- [x] Browser shell, software renderer, hello-world PNG (`target/hello-world.png`)

### Wiring & Integration (Phase 0)

- **Call sites:** `spiral-browser` binary (the end-user entry point) imports from `spiral-core` and `spiral-ipc`.
- **Test coverage:** 143 tests, 100% of `spiral-core` public surface.
- **End-to-end surface:** `./spiral-browser` produces `target/hello-world.png` on a single-process render of `about:blank`.
- **Exit gate:** `cargo build --workspace` green; `cargo test --workspace` green; `target/hello-world.png` produced.

---

## Phase 1 — Engines Foundation 🔄 IN FLIGHT

The minimum-viable engines layer: Vortex executes a `console.log`; Fmt
parses both HTML and CSS from spec; Gyre lays out a block box; Filter
enforces a default policy. Six steps (1.1–1.6) in flight; steps 1.1–1.4
shipped, step 1.5 shipped (per ADR 0001), step 1.6 in progress.

**Owner:** spiral-fmt, spiral-vortex, spiral-gyre, spiral-filter.
**Tests:** see `cargo test --workspace` (live count, verified 2026-06-16; 58 test binaries, 0 failing).

### Step 1.1 — `spiral-crypto` P0 ✅
- [x] `sha2` workspace dep added
- [x] `getrandom` workspace dep added
- [x] Crate skeleton in `crates/spiral-crypto/src/`

### Step 1.2 — Retire `spiral-html` ✅
- [x] `spiral-html` removed from workspace
- [x] All HTML parsing now lives in `spiral-fmt::html`
- [x] `html5ever`, `markup5ever`, `tendril` removed from dependency tree
- [x] Tests for the migration surface

### Step 1.3 — `spiral-fmt` from-spec HTML parser ✅
- [x] 8 insertion modes covered
- [x] Unicode-only input (UTF-8)
- [x] Malformed-input regression tests
- [x] Public entry point: `spiral_fmt::parse_html`

### Step 1.4 — DOM rewire ✅
- [x] `spiral-dom` arena-allocated nodes (Vec<Node> + indices)
- [x] Parent/child relationships via indices
- [x] Attribute storage: `Vec<(String, String)>`
- [x] `Descendants`, `Ancestors`, `NodeDepth` tree-walker API
- [x] Public entry point: `spiral_dom::Dom` (used by `spiral-fmt::html`)

### Step 1.5 — `spiral-fmt` from-spec CSS parser ✅
- [x] CSS Syntax Level 3 tokeniser (8 modules)
- [x] CSS Syntax Level 3 parser (8 modules)
- [x] Selector parser + specificity
- [x] Value parser (lengths, colours, keywords)
- [x] At-rule parser (`@media`, `@font-face`, etc.)
- [x] Declaration parser
- [x] Attribute matcher
- [x] Recovery from errors per CSS Syntax 3 §5
- [x] Public entry point: `spiral_fmt::parse_css`
- [x] `spiral-css` deprecated shim forwards to `spiral_fmt::css::*`
- [x] `cssparser` / `selectors` removed from workspace
- ADR: [0001-css-parser-spiral-fmt](../decisions/0001-css-parser-spiral-fmt.md) (Accepted 2026-06-16)

### Step 1.6 — Phase 1 wrap-up (Vortex slice, HTTP/1.1 stub, filter, Gyre box model) 🔄
> **Historical context:** Step 1.6 was originally the "M4.5
> wrap-up" per the pre-2026-06-16 vocabulary. M-suffix references
> in packet bodies are retained as historical traces per the
> 2026-06-16 SSOT restructure.
- [x] **Packet 1.6.1 (M4.5 Item 8)** — Vortex GC rewrite (per-origin `OriginArena`, `TaggedCell` header, `GcKey` versioning, mark-sweep). 22 new tests.
- [x] **Packet 1.6.2 (M4.5 Item 9)** — Vortex first functional slice (lexer → parser → AST → console.log interpreter). Entry point: `vortex_eval(script: &str) -> Result<JsValue, VortexError>`.
- [x] **Packet 1.6.3 (M4.5 Item 11)** — `spiral-network` HTTP/1.1 client stub. `R: Resolver` generic bound. 1 binary, 1 integration test.
- [x] **Packet 1.6.4 (M4.5 Item 12)** — `spiral-filter` runtime hook (Bet 3). Default policy: "worst offenders only" per `docs/active_context.md` § Ad Policy. ADR: [0005-filter-hook-architecture.md](../decisions/0005-filter-hook-architecture.md).
- [ ] **Packet 1.6.5 (M4.5 Item 13)** — Gyre box model + margins (first Gyre layout work; no Taffy).
- ADR: TBD (packet-level work only)

> **Note (2026-06-16, post-1.6.4 audit):** the original Step 1.6
> also listed Packets 1.6.6 (adoption agency), 1.6.7 (active
> formatting elements), and 1.6.8 (foster parenting) as M4.5
> wrap-up. These were *renumbered* to Step 2.8 (Packets 2.8.1–2.8.3)
> in the Phase 1.5 SSOT restructure and live there now. Step 1.6
> ends at 1.6.5.

### Wiring & Integration (Phase 1)

- **Crates affected:** `spiral-fmt`, `spiral-vortex`, `spiral-gyre`, `spiral-filter`, `spiral-network`, `spiral-dom`, `spiral-css` (deprecated shim), `spiral-html` (retired).
- **Call sites (cross-crate):**
  - `spiral-fmt::html::parse_html` ← `spiral-browser` (Step 1.3 consumer).
  - `spiral-fmt::css::parse_css` ← `spiral-browser` (Step 1.5 consumer).
  - `spiral-dom::Dom` ← `spiral-fmt::html` (Step 1.4 consumer).
  - `spiral-gyre::LayoutEngine` ← `spiral-browser` (Step 1.6.5 consumer).
  - `spiral-vortex::VortexError` ← `spiral-browser` (Step 1.6.2 consumer).
- **Test coverage:** see `cargo test --workspace` (58 test binaries, 0 failing, 2026-06-16).
- **End-to-end surface:** `./scripts/audit-orphan-exports.sh` exits 0 once packets 1.6.2–1.6.5 land. The 23 candidates flagged 2026-06-16 (across 6 crates) are the integration-test surface for this Phase; packets 1.6.1/1.6.3/1.6.4 already closed 11 of the original 34.
- **Exit gate:** `cargo test --workspace` green; `audit-orphan-exports.sh` exits 0; `spiral-fmt::parse_html(html_doc)` returns a `spiral-dom::Dom`; `vortex_eval('console.log("hi")')` returns `Ok(())`. (Packet 1.6.2 ships the second of these exit-gate items — 12 new integration tests prove lex→parse→AST→interpreter end-to-end.)

---

## Phase 1.5 — SSOT Restructure 🔄 IN FLIGHT

The cross-cutting restructure that establishes the SSOT layers documented
in this file. **This is the Phase that creates the tracker you're
reading.** No feature work in this Phase; pure structural.

### Step 1.5.1 — Implementation tracker file ✅ (this file)
- [x] `docs/implementation_tracker.md` created (this file)
- [x] Group→Phase→Step→Packet hierarchy
- [x] Wiring & Integration preamble
- [x] Phase 0 ✅, Phase 1 in flight, Phase 1.5 active, Phase 2+ forward-projected

### Step 1.5.2 — Rule files ✅
- [x] `.spiral/rules/architecture.md` — crate boundaries, ownership rules
- [x] `.spiral/rules/coding-standards.md` — AU English, `?` over `.unwrap()`, import sorting
- [x] `.spiral/rules/testing.md` — TDFlow, isolation, pyramid

### Step 1.5.3 — Role doc expansion ✅
- [x] `docs/agents/security.md` — threat model + 9 audit checklists
- [x] `docs/agents/release.md` — pre-release checklist, SemVer, phase-close protocol
- [x] `docs/agents/onboarding.md` — 60s welcome, read-first sequence, decision tree
- [x] `docs/agents/PROMPT_LIBRARY.md` — canonical prompts

### Step 1.5.4 — CI supply-chain baseline ✅
- [x] `cargo audit` job in `.github/workflows/ci.yml`
- [x] `cargo deny` job in CI with `deny.toml` (license allowlist)
- [x] `gitleaks` job in CI with `.gitleaks.toml`

### Step 1.5.5 — Doc trims and path alignment ✅
- [x] `ROADMAP.md` rewritten as one-page Group→Phase index
- [x] `PLAN.md` §6 (month table) deleted
- [x] `docs/plans/iteration-options.md` §3 / §8.2 / §8.3 deleted; redirect to tracker
- [x] `docs/architecture-shared-everything.md` renamed to `docs/system_architecture.md`
- [x] 3 `design-*.md` files moved to `docs/architecture/design/`
- [x] `docs/innovations-backlog.md` moved to `docs/innovations/backlog.md`
- [x] `docs/phase1-tasks.md` archived to `docs/archives/phase1-tasks.md`

### Step 1.5.6 — SSOT surface normalisation ✅
- [x] `specs/GAP_ANALYSIS.md` status stripped (Deltas 1–7 removed; `[x]/[~]/[ ]` removed)
- [x] GAP_ANALYSIS is now spec-only; P0–P3 priority stack re-tagged onto Phase 2 packets
- [x] `docs/decisions/README.md` index table added; "link from Step" rule
- [x] `docs/active_context.md` cross-refs updated (M4.x → Phase refs)
- [x] `docs/progress_ledger.md` restructure entry appended
- [x] `.editorconfig` added (LF, indent per file type)
- [x] `justfile` recipes: `audit`, `deny`, `wiring`, `release-check`
- [x] `docs/architecture/fmt.md` html5ever references removed
- [x] `docs/releases/0.0.0-bootstrap.md` created (release-notes seed)
- [x] `docs/security/post-mortems/0000-template.md` created (incident template)
- [x] `docs/agents/{implementer,reviewer,architect,tester}.md` updated for new SSOT

### Wiring & Integration (Phase 1.5)

- **Call sites:** Every doc on the `AGENTS.md` read-first path is now reachable; `docs/implementation_tracker.md` is linked from `docs/active_context.md`, `AGENTS.md`, and the new `release.md` / `onboarding.md` role docs.
- **Test coverage:** `cargo build --workspace`, `cargo test --workspace`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings` all green.
- **End-to-end surface:** `./scripts/audit-orphan-exports.sh` still green. New CI jobs (`audit`, `deny`, `gitleaks`) added but **not blocking** on first run; flip to blocking once green.
- **Exit gate:** 14 new files created, 13 edited, 5 moved, 1 renamed. Single ledger entry recording the restructure.

---

## Phase 2 — Engines Depth ☐ NOT STARTED

The top-20 competitive gaps identified in `docs/research/` (worktree
`research/competitive-parity` @ 2026-06-16). Each gap becomes one Step.
**No new feature work; this is the engine-depth phase that turns "parse
real HTML" into "render NYT".** Order is recommended but not enforced;
re-ordering is allowed if a step unblocks another.

Owner: spiral-fmt (HTML), spiral-dom, spiral-vortex (JS stdlib). P0–P3
priority tags from `specs/GAP_ANALYSIS.md` re-tagged onto packets below.

### Step 2.1 — Fragment parsing algorithm
- [ ] **Packet 2.1.1** — Fragment parsing algorithm (WHATWG HTML §12.4). Required for innerHTML, insertAdjacentHTML, template content.
- [ ] **Packet 2.1.2** — Quirk mode classifier (WHATWG HTML §12.1).
- [ ] **Packet 2.1.3** — `<noscript>` element (WHATWG HTML §4.6.7).
- [ ] **Packet 2.1.4** — `<template>` content document-fragment construction.

### Step 2.2 — DOM collection types
- [ ] **Packet 2.2.1** — `NodeList` (live + static variants).
- [ ] **Packet 2.2.2** — `HTMLCollection` (live, named vs indexed access).
- [ ] **Packet 2.2.3** — `DOMTokenList` (classList semantics).
- [ ] **Packet 2.2.4** — `Attr` type and attribute reflection.
- [ ] **Packet 2.2.5** — `NamedNodeMap` (legacy, but required by WPT).
- [ ] **Packet 2.2.6** — `DocumentType` node kind.

### Step 2.3 — Global attributes IDL
- [ ] **Packet 2.3.1** — `id`, `class`, `style`, `title`, `lang`, `dir` reflection.
- [ ] **Packet 2.3.2** — `hidden`, `tabindex`, `contenteditable`, `inert`, `popover`.

### Step 2.4 — `data-*` custom data attributes
- [ ] **Packet 2.4.1** — `dataset` IDL (camelCase mapping).

### Step 2.5 — `globalThis`
- [ ] **Packet 2.5.1** — `globalThis` (ECMA-262 §19.4.1).

### Step 2.6 — `structuredClone`
- [ ] **Packet 2.6.1** — `structuredClone` (WHATWG HTML §8.2.7).

### Step 2.7 — `URL` + `URLSearchParams`
- [ ] **Packet 2.7.1** — `URL` parser (WHATWG URL §4).
- [ ] **Packet 2.7.2** — `URLSearchParams` IDL.

### Step 2.8 — Adoption agency + AFE + foster parenting (deferred from Step 1.6)
- [ ] **Packet 2.8.1** — Adoption agency algorithm (WHATWG HTML §12.2.6.1).
- [ ] **Packet 2.8.2** — Active formatting elements list (WHATWG HTML §12.2.6.1).
- [ ] **Packet 2.8.3** — Foster parenting (WHATWG HTML §12.2.6.1).

### Wiring & Integration (Phase 2)

- **Crates affected:** `spiral-fmt` (HTML), `spiral-dom`, `spiral-vortex` (JS stdlib).
- **Call sites:** `spiral-fmt::html::parse_html_fragment(ctx, html)` consumed by `spiral-dom` setters and by Vortex `Element.innerHTML` setter.
- **Test coverage:** WPT per-packet; one WPT sub-suite per Step.
- **End-to-end surface:** `./justfile wpt-fmt` runs the HTML/WPT subset.

---

## Phase 3 — Networking ☐ NOT STARTED

Owner: spiral-network, spiral-crypto, spiral-ipc, spiral-sandbox.

### Step 3.1 — HTTP/1.1 client
- [ ] **Packet 3.1.1** — `spiral_network::Client::get(url)` returns `Result<Response, NetworkError>`.
- [ ] **Packet 3.1.2** — Redirect handling (≤5 hops).
- [ ] **Packet 3.1.3** — Streaming response body.
- [ ] **Packet 3.1.4** — TLS via rustls (no `native-tls`).
- [ ] **Packet 3.1.5** — Integration test against `httpbin.org`-equivalent fixture.

### Step 3.2 — Cookie jar
- [ ] **Packet 3.2.1** — `CookieJar` with same-site + httpOnly + secure semantics.

### Step 3.3 — DNS resolver
- [ ] **Packet 3.3.1** — `Resolver` trait wrapping `hickory-dns`.

### Step 3.4 — Sandbox profile
- [ ] **Packet 3.4.1** — Linux: Landlock + seccomp-bpf profile.
- [ ] **Packet 3.4.2** — macOS: Seatbelt profile.
- [ ] **Packet 3.4.3** — Windows: Restricted Token.
- [ ] **Packet 3.4.4** — Test that blocked operations fail.

### Step 3.5 — IPC transport hardening
- [ ] **Packet 3.5.1** — Frame-level checksum.
- [ ] **Packet 3.5.2** — Backpressure-aware channel.

### Wiring & Integration (Phase 3)

- **Call sites:** `spiral-network::Client::get` ← `spiral-filter` (default fetch hook).
- **Test coverage:** Integration test against fixture server per Step.
- **End-to-end surface:** `./justfile wpt-network` runs the network/WPT subset.

---

## Phase 4 — Presentation ☐ NOT STARTED

Owner: spiral-render, spiral-ui, spiral-theme.

### Step 4.1 — Vello integration
- [ ] **Packet 4.1.1** — `spiral-vello` workspace member (or vendored fork).
- [ ] **Packet 4.1.2** — GPU adapter selection (wgpu).
- [ ] **Packet 4.1.3** — Display list recording from Gyre output.

### Step 4.2 — Browser chrome
- [ ] **Packet 4.2.1** — Tab bar, URL bar, status bar.
- [ ] **Packet 4.2.2** — Sidebar tabs (Zen-style).
- [ ] **Packet 4.2.3** — Floating URL bar.

### Step 4.3 — Theme system
- [ ] **Packet 4.3.1** — Light/dark mode toggle.
- [ ] **Packet 4.3.2** — System preference detection.

### Wiring & Integration (Phase 4)

- **Call sites:** `spiral-ui` consumes `spiral-render::DisplayList` from `spiral-gyre::LayoutEngine`.
- **End-to-end surface:** `./spiral-browser` opens a window and renders `about:blank`.

---

## Phase 5 — Capability Types Runtime ☐ NOT STARTED

Bet 1 runtime. The type system from Phase 1 (M4 design pass) goes live.

### Step 5.1 — Per-origin isolate
- [ ] **Packet 5.1.1** — `Origin` brand + `OriginHandle` access control.
- [ ] **Packet 5.1.2** — Per-origin DOM/CSSOM/JS globals storage.

### Step 5.2 — Capability tokens in production paths
- [ ] **Packet 5.2.1** — Replace blanket `pub` with `cap::Token`-guarded APIs.

### Wiring & Integration (Phase 5)

- **Call sites:** All cross-crate APIs that previously took `&self` now require a capability token.
- **Test coverage:** Adversarial input fuzz suite per public API.

---

## Phase 6 — Bytecode VM ☐ NOT STARTED

Vortex tree-walker → bytecode VM, gated on real-world profile data.

### Step 6.1 — Bytecode format
- [ ] **Packet 6.1.1** — Bytecode opcodes defined (registers-based, JIT-friendly).
- [ ] **Packet 6.1.2** — Bytecode verifier (type-checks register use).

### Step 6.2 — Stack-based interpreter
- [ ] **Packet 6.2.1** — Tree-walker output → bytecode.
- [ ] **Packet 6.2.2** — Stack-based interpreter.
- [ ] **Packet 6.2.3** — Inline caches (ICs) at call sites.

### Step 6.3 — Profile gate
- [ ] **Packet 6.3.1** — Real-world profile data from Phase 1 site renders.
- [ ] **Packet 6.3.2** — JIT decision: ship tree-walker+VM, defer JIT.

### Wiring & Integration (Phase 6)

- **Call sites:** `vortex_eval` swaps from tree-walker to bytecode VM; same return type.
- **Test coverage:** V8 oracle equivalence suite (already in `v8` feature).

---

## Phase 7 — Media + DRM ☐ NOT STARTED

### Step 7.1 — MSE
- [ ] **Packet 7.1.1** — MSE demuxer scaffolding (MP4, WebM).
- [ ] **Packet 7.1.2** — Source buffer state machine.

### Step 7.2 — Codecs
- [ ] **Packet 7.2.1** — AV1 via `dav1d`.
- [ ] **Packet 7.2.2** — VP9 via `libvpx` or `rav1d`.
- [ ] **Packet 7.2.3** — Opus.
- [ ] **Packet 7.2.4** — AAC.

### Step 7.3 — ClearKey EME
- [ ] **Packet 7.3.1** — ClearKey decryption path.

### Step 7.4 — Widevine (gated)
- [ ] **Packet 7.4.1** — Trust audit of Widevine binary.
- [ ] **Packet 7.4.2** — Widevine CDM bridge (if audit passes).

### Wiring & Integration (Phase 7)

- **End-to-end surface:** A 10s ClearKey-encrypted test video plays in `spiral-browser`.

---

## Phase 8 — Persistent Renderer ☐ NOT STARTED

Bet 4. Vortex heap + layout tree + document checkpoints.

### Step 8.1 — Vortex heap checkpoint
- [ ] **Packet 8.1.1** — `mmap`-friendly heap serialisation.

### Step 8.2 — Layout tree checkpoint
- [ ] **Packet 8.2.1** — Gyre tree serialisation.

### Step 8.3 — Document checkpoint
- [ ] **Packet 8.3.1** — DOM serialisation.

### Wiring & Integration (Phase 8)

- **End-to-end surface:** A warm tab reopens in <100ms via mmap.

---

## Phase 9 — Hardening ☐ NOT STARTED

The CI gates. Not a feature Phase.

### Step 9.1 — Memory budget CI gate
- [ ] **Packet 9.1.1** — Per-phase memory budget enforced in CI.

### Step 9.2 — WPT coverage
- [ ] **Packet 9.2.1** — WPT runner in CI.
- [ ] **Packet 9.2.2** — Per-Phase coverage targets.

### Step 9.3 — Fuzz harnesses
- [ ] **Packet 9.3.1** — Fuzz harness per public parser.
- [ ] **Packet 9.3.2** — Fuzz harness per Vortex bytecode interpreter.

### Step 9.4 — Supply-chain review
- [ ] **Packet 9.4.1** — Every dep has an ADR or a documented reason for exemption.

### Wiring & Integration (Phase 9)

- **End-to-end surface:** A green CI run with all four jobs (`fmt`, `clippy`, `test`, `audit`) passing.

---

## What needs picking (next up)

The next 8 unchecked packets across all phases, in recommended order:

1. **Packet 1.6.5 (M4.5 Item 13)** — Gyre box model + margins.
2. **Packet 2.8.1** — Adoption agency algorithm (WHATWG HTML §12.2.6.1).
3. **Packet 2.8.2** — Active formatting elements list (WHATWG HTML §12.2.6.1).
4. **Packet 2.8.3** — Foster parenting (WHATWG HTML §12.2.6.1).
5. **Packet 2.1.1** — Fragment parsing algorithm (WHATWG HTML §12.4).
6. **Packet 2.7.1** — `URL` parser (WHATWG URL §4).
7. **Packet 2.7.2** — `URLSearchParams` IDL.
8. **Packet 4.1.1** — `spiral-vello` workspace member decision (ADR required).

If you are picking up one of these, **read the linked architecture doc
first** (`docs/architecture/fmt.md`, `vortex.md`, `gyre.md`, `net.md`),
then read the matching ADR if any, then write the failing test, then
write the code (TDFlow).

---

## How to update this file

- **Adding a packet** to an existing Step: edit the Step, append `- [ ] Packet X.Y.N — title`.
- **Closing a packet:** change `[ ]` to `[x]`; do not rewrite.
- **Adding a new Step:** add a `### Step X.N — title` under the parent Phase; add the Step's Wiring & Integration section if it's a major change.
- **Adding a new Phase:** add a `## Phase N — title` block at the end of the Phases list; bump the table.
- **Linking an ADR:** add `ADR: [NNNN-title](../decisions/NNNN-title.md) (Status YYYY-MM-DD)` under the Step or packet.
- **Status update on the Phase:** change the emoji and the wording.

If a Phase is complete, do not delete it. Move its status to `✅ CLOSED
@ vX.Y.Z` once `release.md` ships it.
