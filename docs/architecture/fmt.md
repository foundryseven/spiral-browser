# `spiral-fmt` (Forge) — Format Parsers

> **Brand:** Forge. **Crate:** `spiral-fmt`. **Scope:**
> HTML5 + CSS Syntax Level 3 parsers. **Status:** Phase 1
> Step 1.3 + 1.5 complete (HTML parser + CSS parser shipped).

Forge is Spiral's from-spec format parser. It produces
`spiral_dom::Dom` (HTML) or `spiral_fmt::css::Stylesheet`
(CSS). It is one of three engines in the Phase 1 posture
that carries the Spiral brand (the others: Gyre, Vortex).
See `docs/glossary.md` for the brand table.

---

## Public surface

```rust
// At the crate root.
pub use error::FormatError;
pub fn parse_html(source: &str) -> Result<spiral_dom::Dom, FormatError>;
pub fn parse_css(source: &str) -> Result<css::Stylesheet, FormatError>;
pub use css::{parse as parse_stylesheet, …types};
```

The `css` module itself stays `mod css` (private) to
keep module layout in one place; the public types and
the `parse_stylesheet` alias are re-exported at the
crate root. See `docs/decisions/0001-css-parser-spiral-
fmt.md` for the Fork 1-B rationale.

---

## Internal layout

```
spiral-fmt/src/
├── lib.rs           — crate root, public re-exports
├── error.rs         — FormatError (parse error type)
├── cursor/          — byte-cursor primitive (shared by html and css)
├── token/           — base token type (shared)
├── html/
│   ├── mod.rs       — public parse() entry
│   ├── lexer.rs     — HTML5 tokeniser (8 insertion modes)
│   └── tree.rs      — HTML5 tree builder
└── css/
    ├── mod.rs       — public parse() entry, sub-module re-exports
    ├── parser.rs    — CSS Syntax 3 parser
    ├── selector.rs  — selectors (all 4 combinators, attribute matchers)
    ├── specificity.rs — Selectors Level 4 specificity (a, b, c)
    ├── tokenizer.rs — CSS Syntax 3 tokeniser
    └── value.rs     — CSS values (length, percentage, colour, …)
```

---

## Constraints

- **No `html5ever`, no `markup5ever`, no `tendril`.**
  Pure from-spec Rust. The Phase 1 audit gates this
  explicitly (Phase 1 Step 1.2 retired `spiral-html`).
- **No `cssparser`, no `selectors`, no `cssparser-
  macros`.** Same constraint, same gate. ADR 0001.
- **Output types come from `spiral-dom`.** Forge does
  not own its own DOM type. (The `spiral_dom::Dom` is
  the canonical document model.)
- **Lenient by design.** Both the HTML tree builder
  and the CSS parser recover from errors per the
  relevant spec sections (HTML5 §13.2.6, CSS Syntax
  3 §5).
- **UTF-8 only.** No encoding detection beyond UTF-8
  (yet). Latin-1 / Shift-JIS handling is Phase 2+.

---

## Test posture

- 88 lib tests cover the CSS parser, selectors,
  specificity, values, attribute matchers, and case
  flag.
- 39 e2e tests cover the HTML parser (25) and the CSS
  parser (14) end-to-end.
- The pre-Phase 1 `spiral-html` test corpus (the 6
  previously panicking tests) is fully migrated.

---

## Do-not-touch zones (Phase 1)

- The CSS `parse()` function signature is the public
  contract. Adding parameters is a breaking change.
- The `FormatError` type is `pub` at the crate root;
  the constructor signature is the contract.
- `pub use` re-exports at the crate root are
  `pub`; adding / removing them is a breaking change.

---

## Related

- `docs/decisions/0001-css-parser-spiral-fmt.md` — the
  Fork 1-B decision that moved the CSS parser here.
- `docs/glossary.md` — the Forge brand entry.
- `docs/implementation_tracker.md` § Phase 1 Step 1.3 / 1.5 —
  the canonical status of this crate.
- `AGENTS.md` § `spiral-fmt` — the working rules for
  this crate.
