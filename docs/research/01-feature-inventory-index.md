# Chunk 1 — Core Web Platform Standards Inventory

> **Chunk 1 of 14.** This is the **document and language model** surface
> that every modern browser engine implements. It is the engine-level
> floor: HTML, DOM, CSS, and JavaScript / ECMAScript. **Not** Web APIs
> (Fetch, WebAuthn, Web Storage, Media), **not** networking protocols,
> **not** security mechanisms. Those are chunks 2–8.
>
> **Worktree:** `research/competitive-parity` (base: `audit/m4-window`).
> **Methodology contract:** `00-methodology.md`. **Source ladder:**
> `citations/sources.md`. **Output contract:** `README.md` §"Per-chunk
> output contract".
>
> **MDN Baseline** is the canonical "what does the platform ship" lens;
> per-row evidence is at least one URL, almost always a WHATWG or W3C
> spec plus an MDN page.

---

## Files in this chunk

| File | Topic | Rows | Lines |
|------|-------|------|-------|
| `01-feature-inventory-index.md` (this file) | Header, split rationale, cross-references, open questions | n/a | 168 |
| `01-feature-inventory-html-dom-js.md` | WHATWG HTML (rows 1–83), WHATWG DOM (rows 84–145), ECMAScript (rows 146–223, 248–260), control flow / annexes (rows 224–246), engine-level sub-systems (rows 274–277) | 277 | 528 |
| `01-feature-inventory-css.md` | Selectors (rows 1–21), at-rules (rows 22–37), values & units (rows 38–58), box model (rows 59–81), backgrounds/filters/masks (rows 82–89), text/fonts (rows 90–116), flex (rows 117–127), grid (rows 128–138), transforms/animations (rows 139–152), container queries / drafts (rows 153–159), SVG/MathML CSS (rows 160–168), cascade (rows 169–176), media queries (rows 177–179) | 179 | 328 |

Total: 456 rows across two inventory files. Splitting rule from the
contract (single file ≤ 600 lines) is met by each file independently
(528 and 328 lines respectively).

## How the tables are scored

For every row, seven columns per `README.md` §"Per-chunk output contract":

- **Surface** — `desktop` / `mobile` / `embedded`. Embedded = non-browser
  embedding surfaces (Servo's `WebView`, Chromium `CEF`/`WebView2`,
  WebKit `WKWebView`). For this chunk most rows are "all three".
- **Status in Spiral** — `not-started` / `designed` / `partial` /
  `shipped`. See methodology §3. The wiring rule from
  `AGENTS.md` (audit-orphan-exports.sh) is the bar for `shipped`.
- **Browser prevalence** — methodology §4. Buckets: `>=90%` /
  `75-90%` / `50-75%` / `25-50%` / `<25%` / `behind flag` / `not in any
  shipped browser`. Default for spec-defined modern features is the
  MDN Baseline "Widely available" or "Newly available" bucket.
- **Phase impact** — methodology §5. `P2 / P3 / P4` for most HTML/DOM/CSS
  (vendored parser + layout + DOM-from-JS). `P2 / P3` for most JS in
  Vortex. `P5` for the hot-path work (JIT-grade). Spiral's phase map:
  P2 = core engine, P3 = application shell, P4 = platform (network /
  media / extensions), P5 = advanced.
- **Complexity** — methodology §6. CSS property = `S`; CSS selector
  engine work = `M`; major sub-system (Shadow DOM, Container Queries,
  View Transitions) = `L`; core language feature (async/await, modules)
  = `L`; engine-grade (WASM, threads, SIMD) = `XL`.
- **Engine notes** — one line per reference engine, six engines from
  methodology §2 (Chromium, Gecko, WebKit, Servo, Ladybird, Flow).
  "yes/stable" / "yes/partial" / "behind flag" / "no". Shallow by design
  here; chunk 7 is the engine-coverage deep dive.
- **Sources** — at least one URL. Tier 1 (WHATWG / W3C / IETF / ECMA /
  Unicode) is the spec. Tier 2 (MDN, Can I Use) is the "what do
  browsers actually ship" lens.

## Spiral ground truth (verified 2026-06-16)

| Sub-system | Crate | State | What "shipped" means here |
|------------|-------|-------|---------------------------|
| HTML tokeniser | `spiral-fmt::html::tokeniser` | real code, 8 tokeniser modes incl. Rawtext + ScriptData | wired via `spiral_fmt::parse_html` |
| HTML tree builder | `spiral-fmt::html::tree` | real code, 8 insertion modes | wired via same entry point |
| CSS tokeniser / parser / selectors / specificity / values | `spiral-fmt::css` | real code, 8 modules | wired via `spiral_fmt::parse_css` |
| DOM tree | `spiral-dom` | `Node`, `Element`, `Text`, `Comment`, `Document`, `Dom` + arena | `parse_html` returns it |
| Layout | `spiral-gyre` | `BoxModel`, `EdgeSizes`, `LayoutNode`, `LayoutEngine` stubs | stylesheet parameter currently unused (per `specs/GAP_ANALYSIS.md` 1.4) |
| JS engine (lexer / parser / AST / interpreter / GC / event loop) | `spiral-vortex` | real code, tree-walking interpreter; `EventLoop` with microtasks + macrotasks | in-process only; not yet wired to browser event loop |
| DOM bindings | `spiral-vortex::dom_bindings` | stub (`create_document_object` returns empty `JsObject`) | none |
| JS bytecode VM | `spiral-vortex::vm` | not started (designed for M10–24) | none |

These are the bars for the `Status in Spiral` column on every row.

## Cross-references to `specs/GAP_ANALYSIS.md`

These rows in the GAP file are direct ground truth for the "Status in
Spiral" column and are referenced from the row tables below. Each row in
the inventory cross-links the relevant GAP row.

| GAP section | Title | Status in GAP | What it covers in this chunk |
|-------------|-------|---------------|------------------------------|
| 1.1 | HTML parser (`spiral-html`) | mixed (now superseded — `spiral-fmt` is on disk, `spiral-html` retired 2026-06-15) | HTML §1–§3, §13, §14 |
| 1.2 | CSS parser & cascade | mixed (parser done in M4.4.1 Item 4; cascade deferred) | CSS §1–§2, §4 |
| 1.3 | DOM | partial (Node tree yes, API surface no) | DOM §1, §4–§6 |
| 1.4 | Layout — Gyre | partial (box model yes, rest no) | CSS §3, §6, §8, §9, §14 |
| 1.5 | Render | partial (software renderer; no layout→paint glue) | covered in chunk 9 |
| 1.6 | Vortex — JavaScript engine | partial (lex/parse/interp yes; VM/built-ins/closures partial; DOM bindings stub) | JS §1–§10, §13–§15 |
| 1.7 | Shared-Everything Multi-Process (Bet 1) | partial (types only) | orthogonal to this chunk |

GAP_ANALYSIS rows that fall **outside** chunk 1's scope (chunk-2+):

- §2.1–2.4 (HTTP / TLS / DNS, Crypto, Storage, Image decoder) — chunk 2/6/7
- §4 (Security / Privacy / Filter / GPU / Media / i18n) — chunks 3/5/8/9
- §5 (Process / IPC / WPT / Build) — chunk 10

## Open questions for the user

These surfaced while writing this chunk and need a decision before
chunk 12 (the competitive matrix) is built. They are captured here per
the contract; the inventory below is not blocked on them.

1. **Counting rule for "Prevalence".** The contract says "use the
   bucket definitions in methodology §4", but the gap between
   "85% of page loads in HTTP Archive" and "in all six reference
   engines" is wide. For chunk 1 should prevalence reflect (a)
   MDN Baseline status (canonical web-platform metric), (b) page-load
   frequency in HTTP Archive (real-world usage), or (c) per-engine
   implementation status (matrix column equivalent)? Currently the
   tables use (a) and (c) is deferred to chunk 7.

2. **Element-level coverage cutoff.** The WHATWG HTML spec defines
   ~115 elements, of which ~25 are obsolete but parsable
   (`<acronym>`, `<big>`, `<tt>`, `<marquee>`, etc.). The contract
   says "score the ones that have non-trivial content (not
   `<acronym>`)". Should the obsolete elements (which are present
   in the parser but not in any new content) get a row each, or be
   collapsed into a single row ("HTML5 obsolete elements —
   parser-compatible")? Currently they are collapsed.

3. **Masonry.** CSS Grid Level 3 / Masonry is at the W3C draft
   stage. Should it be in scope for chunk 1, or is it a
   "shapes" / "high-priority draft" item that lives in chunk 8
   (next-gen platform)? Currently it is in scope as a high-priority
   draft per the contract's explicit list.

4. **Intl coverage granularity.** ECMA-402 is large. Should the
   inventory row-per-locale-sensitivity (date / time / number / sort
   / collation / segmentation / display-names / list-format /
   duration-format / relative-time-format / plural-rules /
   segmenter / locale / getCanonicalLocales / supportedValuesOf) or
   row-per-builtin (one row for `Intl.DateTimeFormat` covering all
   its options)? Currently row-per-builtin, with options as a
   nested table.

5. **Engine coverage of the six reference engines.** Some reference
   engines (Flow, Ladybird) are early-stage and implement
   comparatively little of this surface. Where they say "no" is that
   a "no today and on roadmap", a "no with no plan", or simply
   "untested"? The chunk 7 deep-dive needs a richer status than
   "yes/partial/no". Currently "no" is used uniformly.

## Sources

| Tier | Source | URL | Used for |
|------|--------|-----|----------|
| 1 | WHATWG HTML Living Standard | https://html.spec.whatwg.org/ | HTML element, attribute, parser behaviour rows |
| 1 | WHATWG DOM Living Standard | https://dom.spec.whatwg.org/ | DOM interface rows |
| 1 | CSS specifications (W3C / csswg-drafts) | https://www.w3.org/TR/ and https://drafts.csswg.org/ | CSS property / selector / at-rule rows |
| 1 | ECMA-262 | https://tc39.es/ecma262/ | JS syntax, built-ins, semantics |
| 1 | ECMA-402 (Intl) | https://tc39.es/ecma402/ | Intl rows |
| 1 | TC39 stage 3 / stage 4 proposals | https://github.com/tc39/proposals | Modern JS rows |
| 1 | Unicode UTS | https://www.unicode.org/reports/ | Identifier / string normalisation |
| 2 | MDN Web Docs | https://developer.mozilla.org/ | Per-feature reference + Baseline status |
| 2 | Can I Use | https://caniuse.com/ | Per-engine shipping data |
| 2 | webstatus.dev | https://webstatus.dev/ | Baseline core set |
| 2 | WPT (Web Platform Tests) | https://github.com/web-platform-tests/wpt | Conformance evidence |
| 3 | HTML5 lib (portable benchmark) | https://github.com/html5lib/html5lib-tests | HTML parser test corpus |
| 3 | CSS WG test suite | https://github.com/web-platform-tests/wpt/tree/master/css | Conformance evidence |
| 3 | Test262 | https://github.com/tc39/test262 | JS conformance evidence |
| 3 | Servo release notes | https://github.com/servo/servo/blob/main/RELEASES.md | Servo coverage |
| 3 | Ladybird blog / Andreas Kling | https://ladybird.org/ | Ladybird coverage |
| 3 | Flow blog | https://flow.org/blog/ | Flow coverage |

Per-row URLs are inlined in the table cells; this table is the index.
