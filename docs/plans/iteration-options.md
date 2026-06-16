# Iteration Options & Fork Plans

**Status:** Draft — awaiting decision
**Author:** `ozore/custom` (synthesis from prior architecture review)
**Date:** 2026-06-14
**Phase context:** Phase 1 complete (see [`docs/active_context.md`](active_context.md));
this document informs the Phase 2+ dependency strategy.

---

## 0. Purpose

This document is the **decision record** for what to do about each external
dependency in the engine. For every dependency we have one of three postures:

1. **Use as-is** — wrap if needed, never fork.
2. **Vendor / fork** — bring into the workspace, take maintenance responsibility.
3. **Replace** — write our own; treat the existing crate as a reference or
   transitional fallback only.

The default is **(1)**. The bar to switch to (2) or (3) is one of:

- The upstream is dead, abandoned, or about to be.
- We have a *concrete, demonstrable* performance or correctness win that we
  can land and maintain.
- The dependency is a long-term architectural moat we want to own.

The rest of this document evaluates every dependency under that lens and
produces a concrete 12-week plan for the options that pass.

---

## 1. Dependency Triage

| Crate | Upstream | Health | Posture | Rationale |
|-------|----------|--------|---------|-----------|
| `html5ever` | Servo (Mozilla) | **Stale** — last release 2021 | **(2) Vendor → spiral-fmt** | Foundation of HTML parsing; unmaintained; we will own the fixes anyway. MPL-2.0 compatible. |
| `cssparser` | Servo (Mozilla) | **Stale** — last release 2022 | **(2) Vendor → spiral-fmt** | Foundation of CSS parsing and cascade. Same reasoning. |
| `selectors` | Servo (Mozilla) | **Stale** — last release 2022 | **(2) Vendor → spiral-fmt** | Selector matching is half the cascade engine. Same reasoning. |
| `taffy` | Dioxus | Healthy, active | **(3) Replace** with custom layout — Gyre | We need WebKit/Blink-class flex/grid edge cases. Taffy is solid but not at parity. Gyre is in-house from day one; no transitional Taffy dep. |
| `vello` | Linebender | Active | **(1) Use as-is**; later **(2) Fork** for tile-based scrolling | Beautiful compute renderer, but no picture cache / dirty-rect optimisations. Phase 4 work. |
| `wgpu` | Linebender | Active | **(1) Use as-is** | Right abstraction. Don't touch. |
| `harfrust` | Bushel | Active, newer than HarfBuzz | **(1) Use as-is** | Pure-Rust HarfBuzz port. Less proven than upstream HarfBuzz but feature coverage is converging. |
| `swash` | Linebender | Active | **(1) Use as-is** | Clean glyph cache, good defaults. |
| `cosmic-text` | pop-os | Active | **(1) Use as-is** | Full text pipeline (shaper + rasteriser + layout). |
| `boa_engine` | Boa | Active, interpreter only | **(3) Replace** with `rusty_v8` via Vortex | Interpreter-only JS is unrecoverable for a real browser. The single biggest gap. Decision (2026-06-14): skip `rquickjs` intermediate; go straight to V8. |
| `hyper` | hyperium | Active | **(1) Use as-is** | Production-grade. Wrap in `spiral-network::Client`. |
| `rustls` | rustls | Active | **(1) Use as-is** | Memory-safe TLS. Wrap in `spiral-net`. |
| `hickory-dns` | hickory | Active | **(1) Use as-is** | Wrap in `spiral-net::Resolver`. |
| `serde` / `bincode` | dtolnay / message-io | Active | **(1) Use as-is** | No reason to fork serialisation. |
| `tokio` | tokio-rs | Active | **(1) Use as-is** | Standard. |
| `winit` | rust-windowing | Active | **(1) Use as-is** | Standard. |
| `png`, `zune-jpeg`, `webp`, `ravif` | various | Active | **(1) Use as-is** | Each format has one canonical Rust crate. |
| `thiserror`, `anyhow` | dtolnay | Active | **(1) Use as-is** | Standard. |
| `criterion` | criterion-rs | Active | **(1) Use as-is** | Standard benchmarking. |

**Net result:** two crates to vendor, one to replace with custom work, one to
replace with a different crate, and one to fork later (vello, in Phase 4).
Everything else stays.

---

## 2. The Five Concrete Plans

The five options below are ordered by **leverage** (payoff per engineer-week).
Each is fully scoped: deliverables, exit criteria, risks, dependencies, and an
estimated calendar.

---

### Option A — Vendor the Servo parsers into `spiral-fmt`

**Goal:** Take ownership of `html5ever`, `cssparser`, and `selectors` by
bringing them into the workspace as a single unified crate, modernising the
codebase, and exposing a clean facade.

**Posture:** (2) Vendor

**Crate:** new — `crates/spiral-fmt/`

**Upstream repos:**
- `https://github.com/servo/html5ever`
- `https://github.com/servo/rust-cssparser`
- `https://github.com/servo/servo/tree/main/components/selectors` (siblings)

**License:** MPL-2.0 (file-level copyleft, compatible with our MPL-2.0).

**Why now:**
- Both crates are unmaintained. We will be writing patches; we might as well
  own the code.
- Spec gaps (CSS nesting, container queries, `:has()`, `:focus-visible`)
  require changes too sweeping for a stale upstream to accept.
- Vendoring removes the risk that an upstream force-push or relicense breaks us.

**Deliverables:**
1. `crates/spiral-fmt/Cargo.toml` with three vendored sub-crates or a single
   unified crate. Recommendation: one crate, three modules.
2. `crates/spiral-fmt/src/html.rs` — ported `html5ever` markup parser.
3. `crates/spiral-fmt/src/css.rs` — ported `cssparser` tokeniser + parser.
4. `crates/spiral-fmt/src/select.rs` — ported `selectors` matching.
5. `crates/spiral-fmt/src/lib.rs` — unified facade:
   ```rust
   pub fn parse_html(input: &str) -> Result<spiral_dom::Document, FormatError>;
   pub fn parse_css(input: &str) -> Result<Stylesheet, FormatError>;
   pub fn match_selector(elem: &Element, sel: &Selector) -> bool;
   ```
6. Modernised dependency stack:
   - `tendril` → `compact_str`
   - `string_cache` → `string_cache_plus` (or owned `Atom`)
   - `phf` macros → `const_phf` or const-eval tables
   - `markup5ever` interfaces → plain `spiral-dom` types
7. CI: `cargo build`, `cargo test`, `cargo clippy -D warnings`,
   `cargo fmt --check` on the new crate.
8. WPT fixture harness for HTML and CSS — at least 50 HTML5 lib tests
   passing, at least 100 CSS parser tests passing.

**Exit criteria:**
- `spiral-html` and `spiral-css` depend on `spiral-fmt` (not the Servo crates).
- `cargo test --workspace` is green.
- `cargo tree | grep -E 'servo|html5ever|cssparser|selectors'` returns
  nothing outside `spiral-fmt`.
- The original `html5ever` HTML5 lib test suite (subset we port) passes.
- A malformed-input corpus of 10,000 fuzz cases does not panic.

**Risks:**
- **Size.** `html5ever` alone is ~15k LOC. Three crates combined are ~35k LOC
  of unfamiliar code to absorb. Mitigation: minimal diff in the first pass,
  modernisation in a follow-up sprint.
- **Test coverage loss.** Upstream tests are the contract. We must keep them
  passing as we modernise, or add our own.
- **Build time.** Vendored crates can balloon compile times. Mitigation: keep
  `cfg(test)` test targets slim; use `cargo nextest` for parallelism.

**Dependencies:** none. Can start in parallel with Phase 2 layout work.

**Effort estimate:** 4–6 weeks single engineer; 2–3 weeks with two engineers
working on html5ever and cssparser in parallel.

**Payoff:** Removes the largest single point of architectural rot. Foundation
for any future CSS spec work.

---

### Option B — Replace Taffy with a custom layout engine

**Goal:** Implement block, flex, and grid layout in `spiral-gyre` (Gyre) from
scratch, reaching WPT parity for the common cases. Gyre is in-house from
day one; no Taffy in the tree, no transitional fallback.

**Posture:** (3) Replace

**Crate:** `crates/spiral-gyre/`

**Why now:**
- Layout is the layer where we can produce something *measurably better*
  than the dependency. Edge cases in flex (`min-content`, `auto` margins
  in overflow, baseline alignment) and grid (`subgrid`, masonry) are
  weak in Taffy.
- A custom layout engine is the single highest-prestige piece of work in
  a browser. It is the part of the engine that defines us.
- We already own block layout (Sprint 5 will land it). Flex and grid are
  the natural next steps.

**Deliverables (sequenced):**

**Phase 2a — Block (Months 4–7):**
1. Box model: `margin`, `border`, `padding`, `content` (already designed).
2. Normal flow: block stacking, margin collapse (positive/negative/nested).
3. BFC and IFC.
4. Floats (left/right, clear, BFC containment).
5. Positioning: `static`, `relative`, `absolute`, `fixed`, `sticky`.
6. WPT pass target: **40%** of `css/css-box/` and `css/css-position/`.

**Phase 2b — Flex (Months 7–10):**
1. Flex container model: main/cross axis, flex lines.
2. `flex-direction`, `flex-wrap`, `flex-flow`.
3. `justify-content`, `align-items`, `align-content`, `align-self`.
4. `flex-grow`, `flex-shrink`, `flex-basis`, `min-width`/`max-width` interaction.
5. `order`, baseline alignment.
6. WPT pass target: **60%** of `css/css-flexbox/`.

**Phase 2c — Grid (Months 11–15):**
1. Grid container: explicit and implicit tracks.
2. `grid-template-columns/rows`, `grid-template-areas`.
3. Line-based placement, span, named lines.
4. `grid-auto-flow: row/column/dense`.
5. `grid-gap` (and the split `row-gap`/`column-gap`).
6. `subgrid` (Level 2).
7. WPT pass target: **40%** of `css/css-grid/` (full parity is a v0.3+ goal).

**Phase 2d — (N/A):** Taffy was never added. Gyre is in-house from the first
WPT fixture. The "remove Taffy" milestone is gone from the plan.

**Exit criteria:**
- WPT pass rate ≥ 40% (block) + 60% (flex) + 40% (grid) of the relevant
  suites.
- `cargo tree | grep taffy` returns nothing — Taffy is never added.
- `benches/layout/` shows our engine is within 2× of Taffy on representative
  pages, and within 1.5× on pages without grid.

**Risks:**
- **Time.** Layout is the bulk of the work between today and a usable browser.
  WPT parity for flex alone is measured in engineer-years historically.
- **Spec bugs.** Every layout engine has a long tail of obscure bugs. We will
  too. The mitigation is test-driven: every WPT failure becomes a test first.
- **Performance.** Custom layout is easy to write and easy to make slow.
  Bench continuously from day one.

**Dependencies:** None. Independent of Option A, but Option A's clean DOM
types will make this easier.

**Effort estimate:** 12–18 months single engineer; 6–9 months with 2–3 layout
specialists.

**Payoff:** This is the *only* layer where Spiral can produce something
genuinely different from Taffy, and where bugs are most user-visible. The
prestige-and-correctness moat.

---

### Option C — Fork Vello and add tile-based picture caching

**Goal:** Add scroll-friendly optimisations to Vello: picture caching, tile
rendering, and a "low-power" path for static content.

**Posture:** (1) Use as-is now; **(2) Fork** in Phase 4

**Crate:** new — `crates/spiral-vello/` (fork of `vello`)

**Why later, not now:**
- Vello is the right primitive for full-scene compute rendering. We should
  not preemptively fork before we have evidence we need to.
- The win is in **incremental** rendering — caching the painted page into
  tiles so that scrolling only re-blits, not re-paints. This is a Phase 4
  concern (we are not even GPU-rendering until Phase 4).

**Deliverables:**
1. Tile-based scene subdivision (configurable tile size, default 256×256).
2. Picture cache: stable display lists are rendered once and cached as
   textures.
3. Dirty-rect invalidation: only re-render tiles whose inputs changed.
4. "Low-power" path: a CPU fallback for static pages (similar to our current
   `SoftwareRenderer` in `spiral-render`).
5. `bench/render/` benchmarks showing scrolling at 120fps for a representative
   page.

**Exit criteria:**
- Scrolling a 4K page at 120fps on integrated graphics.
- Memory overhead of cache < 200 MB for typical pages.
- API stays compatible with upstream Vello for the 80% case.

**Risks:**
- **Upstream divergence.** A Vello fork is a long-term maintenance tax. If
  upstream adds a feature we need, we rebase. Mitigation: keep the diff
  minimal; push improvements upstream first.
- **GPU driver bugs.** Tile-based rendering exposes driver bugs that
  full-scene does not. Mitigation: have a CPU fallback that always works.

**Dependencies:** Phase 4 work. Blocked on Vello becoming a bottleneck in
real-world use.

**Effort estimate:** 3–4 months. Smaller than B because the rendering
correctness already exists in Vello; this is optimisation, not greenfield.

**Payoff:** "Spiral feels faster than Chrome" is a real pitch. This is the
engineering that produces it.

---

### Option D — Replace Boa with a from-scratch JS engine (DECIDED 2026-06-14)

**Goal:** Build Vortex — Spiral's own JavaScript engine from the ground up
in safe Rust. No V8, no QuickJS, no Boa. Google's V8 (`rusty_v8`) stays
behind a `v8` feature flag as a CI compliance oracle only.

**Posture:** (3) Replace

**Crate:** `crates/spiral-vortex/`

**Decision record (2026-06-14):**
- Vortex is a from-scratch Rust JS engine: lexer, parser, AST, bytecode
  compiler, stack VM, mark-sweep GC, future baseline JIT.
- `rusty_v8` is gated behind `v8` feature (off by default). The CI test
  harness runs JS snippets through both Vortex and V8 and compares outputs.
- Roadmap stretched to 6–8 years to accommodate building a JS engine
  alongside the rest of the browser.
- This matches the Ladybird/LibJS posture: own the JS interpretation
  layer end-to-end.

**Why this choice over V8:**
- Independence: Spiral owns its JS engine the same way it owns Gyre.
- Security: memory-safe Rust throughout; no C++ attack surface from V8.
- Brand: "Vortex" is Spiral's engine, not a Google product we embed.
- V8 oracle ensures correctness during development without accepting
  V8 as a permanent dependency.

**Phased Vortex roadmap:**

**Phase A (Months 4–9): Tree-walking interpreter.**
1. Lexer: full ECMAScript tokeniser (all operators, keywords, literals).
2. Parser: recursive descent with Pratt parsing for expressions.
3. AST: comprehensive node types for ES2015+ syntax.
4. Tree-walking interpreter: Environment (scope chain), exec_stmt, eval_expr.
5. `console.log/info/warn/error` → `RendererToBrowser::ConsoleMessage`.
6. ~5–10% Test262 pass rate.

**Phase B (Months 10–24): Bytecode VM + ES2015+ syntax.**
1. Bytecode compiler: AST → bytecode instructions.
2. Stack-based VM: execute bytecode (replaces tree-walker, ~5–10× faster).
3. Closures, prototypes, `this`, classes, arrow functions.
4. ES2015 syntax: let/const, template literals, destructuring, spread.
5. Promises, async/await, generators.
6. Builtins: Object, Array, String, Number, Boolean, Math, JSON, Date, RegExp.
7. Mark-sweep GC with roots tracing from stack + globals.
8. DOM bindings: createElement, appendChild, setAttribute, etc.
9. Event dispatch skeleton: addEventListener, dispatchEvent.
10. ~30–40% Test262 pass rate.

**Phase C (Months 25–42): Baseline JIT + builtins.**
1. Cranelift-based baseline JIT for hot functions.
2. Type feedback, inline caches for property access.
3. Map, Set, WeakMap, WeakSet, Symbol, iterators, generators.
4. ES modules (import/export).
5. Full DOM API: removeChild, textContent, style, event objects.
6. ~60% Test262 pass rate.

**Phase D (Months 43–60): Optimising JIT.**
1. Optimising tier with speculative optimisations.
2. Concurrent/incremental GC.
3. WebAssembly support.
4. ~80% Test262 pass rate.

**Exit criteria for v0.1 (Month 60):**
- Vortex bytecode VM + baseline JIT operational.
- ~80% Test262 pass rate.
- `cargo tree | grep rusty_v8` returns nothing for default build.
- A test page that mutates the DOM from JS produces a re-layout.
- Console output appears in DevTools.
- V8 oracle passes CI compliance suite.

**Risks:**
- **Scope.** Building a JS engine is a multi-year project. Mitigation:
  phased roadmap; LLM-assisted development compresses timelines.
- **Test262 compliance.** The ECMAScript spec is 800+ pages. Mitigation:
  V8 oracle provides a correctness reference during development.
- **Performance.** A from-scratch engine won't match V8's 20 years of
  optimisation work. Mitigation: bytecode VM + baseline JIT gets us
  to "good enough for real sites"; optimising JIT is Phase D.

**Effort estimate:** 6–8 years total for production-quality Vortex.
Phase A (interpreter) is ~6 months. Phase B (bytecode VM) is ~15 months.
Phase C (JIT) is ~18 months. Phase D (optimising JIT) is ~18 months.

---

### Option E — Wrap and integrate (do not fork)

**Goal:** Document the dependency surface and add thin Spiral-specific
wrappers where they buy us decoupling.

**Posture:** (1) Use as-is

**Crates:** `spiral-network`, `spiral-net`, `spiral-imagedecoder`, `spiral-crypto`,
`spiral-ipc` (already done).

**Why this exists as a plan:**
The temptation in an early project is to fork everything. The result is
that you own everything, including security CVEs in `hyper` that get fixed
upstream and don't reach you. This plan is the explicit "we are *not*
forking these, and here is why" record.

**Wrappers to add:**

1. `spiral_net::Resolver` wrapping `hickory_resolver::TokioResolver`.
   - Hides the resolver configuration behind a `BrowserConfig::dns_servers`
     knob.
   - Trait-abstracted for testing with a mock resolver.

2. `spiral_net::TlsConnector` wrapping `rustls::ClientConfig`.
   - Single source of truth for the system trust store.
   - Hook for certificate pinning (Phase 5).

3. `spiral_network::Client` wrapping `hyper::Client`.
   - Connection pooling.
   - Cookie jar integration.
   - Redirect policy.
   - Request/response interceptors (for DevTools).

4. `spiral_imagedecoder::Decoder` enum dispatching to per-format crates.
   - One entry point: `Decoder::decode(bytes, format) -> Image`.
   - Lazy progressive decoding.

5. `spiral_crypto::SecureRandom` wrapping `rand::rngs::OsRng`.
   - One trait, one default impl. Lets us swap in `getrandom` directly
     without breaking the API later.

**Exit criteria:**
- All public Spiral APIs go through the wrapper, not the upstream crate
  directly.
- `cargo doc --workspace` shows a clean Spiral-namespaced surface.
- No business-logic crate (anything below `spiral-browser`) imports
  `hyper`, `rustls`, `hickory_*` directly.

**Risks:** None significant. This is hygiene.

**Effort estimate:** 1–2 weeks per wrapper, parallelisable.

**Payoff:** Decoupling. We can swap `hyper` for `reqwest` (or a future
replacement) in one place. We own the security perimeter of the trust
store and the cookie jar.

---

## 3. Sequencing — Recommended 12-Week Plan

This is the plan I'd execute starting next week. It assumes Phase 1 is
complete (it is) and that we have 1–2 engineers available.

| Week | Tracks (parallel) | Crate | Goal |
|------|-------------------|-------|------|
| 1 | A1: Vendor `html5ever` into `spiral-fmt` | `spiral-fmt` (new) | Code in tree, builds |
| 1 | A2: Vendor `cssparser` into `spiral-fmt` | `spiral-fmt` (new) | Code in tree, builds |
| 1 | E1: Design `spiral_net::Resolver` trait | `spiral-net` | API stabilised |
| 2 | A1: Modernise `html5ever` deps (tendril → compact_str, etc.) | `spiral-fmt` | Lints clean |
| 2 | A2: Modernise `cssparser` deps | `spiral-fmt` | Lints clean |
| 2 | E2: Design `spiral_net::TlsConnector` trait | `spiral-net` | API stabilised |
| 3 | A3: Vendor `selectors` into `spiral-fmt` | `spiral-fmt` | Code in tree, builds |
| 3 | A4: Unified facade `spiral_fmt::parse_html` / `parse_css` | `spiral-fmt` | API stable |
| 3 | E3: `spiral_network::Client` trait | `spiral-network` | API stabilised |
| 4 | A5: Port original html5ever HTML5 lib tests; subset must pass | `spiral-fmt` | Test contract preserved |
| 4 | A6: Port original cssparser tests; subset must pass | `spiral-fmt` | Test contract preserved |
| 4 | D1: Spike — `rquickjs` hello world in `spiral-js` | `spiral-js` | Engine chosen |
| 5 | A7: `spiral-html` rewires to `spiral-fmt` | `spiral-html` | Servo dep gone from `spiral-html` |
| 5 | A8: `spiral-css` rewires to `spiral-fmt` | `spiral-css` | Servo deps gone from `spiral-css` |
| 5 | B1: `spiral-layout` block layout — first pass | `spiral-layout` | Roadmap Month 7 |
| 6 | A9: Fuzz harness for `parse_html`, `parse_css` | `spiral-fmt` | No panics on 10k corpus |
| 6 | D2: `rquickjs` → `console.log` → `RendererToBrowser` | `spiral-js` | Console pipe live |
| 6–8 | B2: Block layout — floats, BFC, margin collapse | `spiral-layout` | Roadmap Months 7–8 |
| 6–8 | E4: `spiral_imagedecoder::Decoder` enum | `spiral-imagedecoder` | Roadmap Month 18 (front-loaded) |
| 9–10 | D3: `rquickjs` DOM bindings — `createElement`, `appendChild`, `setAttribute` | `spiral-js` | DOM-mutate works |
| 9–12 | B3: Flex layout — first pass, no Taffy | `spiral-layout` | Roadmap Month 8 |
| 11–12 | D4: `trait JSRuntime` abstraction in `spiral-js` | `spiral-js` | Engine-swap path open |
| 11–12 | B4: WPT fixtures for block layout | `spiral-layout` | Test harness operational |

**End of week 12:**
- Servo parser crates are gone from `spiral-html` and `spiral-css`.
- `taffy` is still in the tree, on track for removal at month 18.
- `boa_engine` is no longer in the plan; `rquickjs` is.
- `spiral-net`, `spiral-network`, `spiral-imagedecoder` have clean wrapper
  APIs.
- Block layout is shipping its first WPT-federated milestone.

---

## 4. Decision Matrix

| Option | Effort | Payoff | Risk | Verdict |
|--------|--------|--------|------|---------|
| A — Vendor Servo parsers | Medium | High (foundational) | Medium | **Do it now** |
| B — Replace Taffy with custom layout | Very high | Very high (the moat) | High | **Roadmap commitment** |
| C — Fork Vello | Medium | High (perf differentiator) | Medium | **Phase 4 work** |
| D — Replace Boa with rquickjs (→ V8) | Medium | Very high (JS gap) | Medium | **Do it at Phase 3 start** |
| E — Wrap and integrate | Low | Low (hygiene) | None | **Do it always** |

**My recommendation:** A, D, and E in parallel over the next 12 weeks. B as
a multi-quarter roadmap commitment. C deferred to Phase 4.

---

## 5. What we are explicitly *not* doing

- **Not forking `wgpu`.** The right abstraction.
- **Not forking `hyper`.** Production-grade, wrapped.
- **Not forking `rustls`.** Memory-safe TLS, wrapped.
- **Not forking `harfrust`.** Active, converging on HarfBuzz feature parity.
- **Not forking `taffy` and trying to maintain it.** We replace it; we don't
  own it.
- **Not writing a custom HTTP client, DNS resolver, or TLS stack.** Use
  upstream, wrap, ship.
- **Not building a custom crypto primitive.** `rustls` and the platform
  `getrandom` are the answer.
- **Not building a custom serialisation format.** `bincode` is fine.
- **Not owning upstream's security CVE backlog.** Vendor only the crates
  that are actually dead.

---

## 6. Open Questions

1. **Layout team.** Who works on Option B? This is multi-quarter. Is there
   one engineer with full bandwidth, or do we split between layout and the
   other tracks?
2. **V8 binary size.** Are we willing to ship a 30 MB browser binary for v0.2
   in exchange for real JS performance? Or is `rquickjs` good enough that we
   never need V8?
3. **Vendoring cost.** Is the 4–6 week vendor effort on `html5ever`/
   `cssparser` worth the maintenance tax, or do we push the Servo repos
   to a "Spiro-fork" GitHub org and submit PRs upstream (which will not
   be reviewed)?
4. **Vello fork timing.** Do we wait for empirical scroll jank in our own
   Phase 4 work, or preemptively fork now on the theory that we will need
   it?

Decisions on (1) and (2) gate the roadmap. (3) and (4) can defer to the
sprint that picks them up.

---

## 7. SSOT Links

- [`docs/active_context.md`](active_context.md) — current sprint state
- [`docs/progress_ledger.md`](progress_ledger.md) — change log
- [`docs/system_architecture.md`](system_architecture.md) — architecture deltas
- [`docs/phase1-tasks.md`](phase1-tasks.md) — completed Phase 1 task breakdown
- [`ROADMAP.md`](../ROADMAP.md) — phase plan
- [`ARCHITECTURE.md`](../ARCHITECTURE.md) — canonical architecture
