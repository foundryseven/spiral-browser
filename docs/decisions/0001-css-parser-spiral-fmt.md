# ADR 0001: CSS Syntax 3 parser moves to `spiral-fmt::css`; `spiral-css` becomes a deprecated shim (Fork 1-B)

**Status:** Accepted
**Date:** 2026-06-16
**Deciders:** James Pinnell, the architect (Fork 1-B call)
**Related:** `docs/audits/2026-06-15-baseline.md`, `docs/progress_ledger.md` (M4.4.1 Item 4 entry), `specs/GAP_ANALYSIS.md` Delta 4, `AGENTS.md` `spiral-fmt` section

---

## Context

`spiral-css` was originally a thin wrapper over the `cssparser`
and `selectors` workspace crates — a hand-rolled CSS parser and
selector engine. Two problems with this approach:

1. **G1.2 (the last open M4.4.1 stub).** The audit at
   `specs/GAP_ANALYSIS.md:65` recorded the from-spec CSS
   parser in `spiral-fmt` as `[ ]` (pending). The minimum-viable
   HTML parser (Chunks 1–3) had been moved to `spiral-fmt::html`
   and the `spiral-html` crate retired 2026-06-15, but CSS
   parsing still lived in `spiral-css` with `cssparser` /
   `selectors` workspace deps. The result: two non-conforming
   parser surfaces, and a missing from-spec CSS implementation
   to anchor M5+ (cascade, shorthand expansion, `@media`
   matching).

2. **Crate split was a layering violation.** `spiral-fmt` is
   the project's parser crate (`html`, `css`, `error`,
   `cursor`, `token`). Splitting CSS off into its own crate
   meant the two format-parsing subsystems lived in different
   workspaces, with different design conventions, different
   test patterns, and different dependency footprints. New
   code couldn't share infrastructure between HTML and CSS.

The HTML migration (Chunks 1–3) had already established the
pattern: write a from-spec parser in `spiral-fmt::html`,
retire the old `spiral-html` crate, keep any external
consumers alive via a deprecated shim. CSS needed the same
shape.

---

## Decision

Fork 1-B:

1. Implement a from-spec CSS Syntax Level 3 tokeniser and
   parser in `spiral-fmt::css`. Pure Rust, no `cssparser`,
   no `selectors`.
2. Re-export the public types at the `spiral_fmt` crate root
   (`parse_css`, `Stylesheet`, `Rule`, `QualifiedRule`, `AtRule`,
   `AtBlock`, `Declaration`, `SelectorList`, `ComplexSelector`,
   `ComplexSelectorStep`, `CompoundSelector`, `TypeSelector`,
   `Combinator`, `AttributeSelector`, `AttributeMatcher`,
   `AttributeCase`, `Specificity`, `Value`).
3. Convert `spiral-css` into a `#[deprecated]` re-export shim.
   The shim's `lib.rs` re-exports the new types verbatim, and
   provides a `CssParser` adapter that calls
   `spiral_fmt::parse_css` and stores the result.
4. Drop the `cssparser` and `selectors` workspace deps from
   `spiral-css/Cargo.toml`. Add `spiral-fmt` as the replacement
   dep.
5. Keep `use spiral_css::Stylesheet` working — `spiral-gyre`
   (the only consumer at `crates/spiral-gyre/src/lib.rs:9`)
   must compile unchanged against the shim.

The shim is the migration boundary. New code calls
`spiral_fmt::parse_css` directly. Old code keeps working
through the shim, with a `#[deprecated]` lint pointing at
the new path. The shim is **not** a deletion — it is a
deprecation. The old API surface stays; the build stays
green; the migration is a search-and-replace, not a forced
rewrite.

---

## Consequences

- **Positive:**
  - One parser crate (`spiral-fmt`) for both HTML and CSS.
    Shared error type, shared cursor, shared tokenisation
    primitives.
  - From-spec implementation. No MPL-2.0 vendored code, no
    upstream surprises. The `spiral-html`-retirement
    precedent is mirrored.
  - `spiral-gyre` and any future consumer can adopt
    `spiral_fmt::parse_css` at their own pace. The shim
    keeps them building.
  - G1.2 closed. The M4.4.1 minimum-viable parser scope
    (qualifiers, at-rules, selectors with all combinators
    and attribute matchers, specificity, `!important`) is
    green.
- **Negative:**
  - `spiral-css` becomes a permanent shim until every
    consumer migrates. The `#[deprecated]` lint is
    `#[allow]`-able but not removable.
  - The shim adds a layer of indirection at the call site.
    Small cost, but a step further from the source.
- **Migration:**
  - Search: `spiral_css::parse`, `spiral_css::Stylesheet`,
    `spiral_css::CssParser`, `spiral_css::CssValue`,
    `spiral_css::CssProperty`, `spiral_css::CssRule`,
    `spiral_css::Selector`, `spiral_css::SelectorPart`.
  - Replace with the corresponding `spiral_fmt::css::*` or
    `spiral_fmt::*` symbol.
  - Remove the `use spiral_css::…` line; add a
    `use spiral_fmt::…` line.
  - Drop `spiral-css` from the `Cargo.toml` workspace dep
    when nothing in the crate depends on it any more.

---

## Alternatives considered

### Option A: Continue with `spiral-css` + `cssparser`/`selectors`

Rejected because it leaves G1.2 open and the two parser
surfaces in different crates. Same problem that triggered
the `spiral-html` retirement in Chunks 1–3.

### Option B: Implement a CSS parser in `spiral-fmt::css` AND delete `spiral-css`

Rejected because `spiral-gyre` (the layout engine) imports
`spiral_css::Stylesheet` at `crates/spiral-gyre/src/lib.rs:9`
and currently uses an empty `Stylesheet` as a placeholder
until M4.6 wires real styles into the layout pipeline. A
hard delete would force a `spiral-gyre` rewrite that has
nothing to do with the M4.4.1 minimum-viable parser scope.
A shim is cheaper and lower risk.

### Option C: Move CSS to `spiral-fmt::css` but make `spiral-css` a hard fork of the new types (no re-exports)

Rejected because the `spiral-css` shim's job is to keep
external consumers building, not to be a parallel type
hierarchy. A re-export shim is the simplest path that
preserves the `spiral_css::Stylesheet` symbol name.

---

## Wiring & Integration

Per the project rule (see `AGENTS.md`): a decision is not
done until its outcome is reachable from a real surface.
This decision's outcome — the new CSS parser — is wired
as follows:

- **Crates affected:**
  - `spiral-fmt` — adds `src/css/{mod,parser,selector,
    specificity,tokenizer,value}.rs`. Public types and
    `parse_css` re-exported at the crate root.
  - `spiral-css` — `lib.rs` rewritten as a `#[deprecated]`
    re-export shim with a `CssParser` adapter. `Cargo.toml`
    drops `cssparser` + `selectors`, adds `spiral-fmt`.
- **Call sites:**
  - `spiral_fmt::parse_css(&str) -> Result<Stylesheet, _>`
    is the new entry point. Exposed at the crate root and
    through `spiral_css::parse_css` (deprecated alias).
  - `spiral_css::CssParser::parse(&str) -> Result<()>`
    stores the result internally; `stylesheet()` returns
    a `&Stylesheet`.
- **Test coverage:**
  - 88 lib tests in `spiral-fmt::css` cover the parser,
    selectors, specificity, values, attribute matchers,
    case flag, and all six combinators.
  - 14 new e2e tests in `crates/spiral-fmt/tests/e2e.rs`
    cover qualified rules, at-rules (block and `;`
    terminator), specificity comparisons, attribute
    selectors with the `i` flag, pseudo-class, and
    `!important`.
  - 2 lib tests in the `spiral-css` shim cover the
    `CssParser` round-trip and the empty default state.
  - `spiral-gyre` still compiles (the `use spiral_css
    ::Stylesheet` import at `crates/spiral-gyre/src/lib.rs:9`
    resolves through the shim). The layout pipeline is not
    wired to the new parser yet — that is M4.6 (Gyre
    stylesheet integration).
- **Reachable from a real surface:** yes. The new parser
  is reachable from the public entry point
  `spiral_fmt::parse_css`, from the deprecated shim
  `spiral_css::parse_css`, and from 16 tests across two
  test binaries. There are **no orphan exports**.

---

## Notes

The `cssparser` and `selectors` workspace deps in the
top-level `Cargo.toml` are still declared for now (lines
68–69). They are unused by any crate. They should be
removed in a follow-up commit that touches the workspace
manifest only. Not done in this commit to keep the
surface area of the CSS-parser migration small.
