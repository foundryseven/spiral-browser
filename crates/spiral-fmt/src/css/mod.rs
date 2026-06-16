//! Spiral Browser — CSS parser.
//!
//! The M4.4.1 minimum-viable CSS parser lives here. It is
//! a from-spec implementation of the CSS Syntax Level 3
//! tokeniser and parser, restricted to the surface the
//! M4.4.1 test corpus exercises:
//!
//! - CSS3 tokeniser (identifiers, numbers, percentages,
//!   dimensions, strings, hashes, at-keywords, all the
//!   punctuation the selector / declaration syntax needs)
//! - Qualified rules (`selector { decl; decl; }`)
//! - At-rules with both `{ }` block form (`@media`,
//!   `@supports`, `@font-face`, `@keyframes`, `@layer`)
//!   and `;` terminator form (`@import`, `@charset`,
//!   `@namespace`)
//! - Selectors: type, universal, class, ID, attribute
//!   (with all six matchers plus the `i` / `s` case
//!   flags), pseudo-class, pseudo-element, all four
//!   combinators
//! - Specificity per Selectors Level 4
//! - Values: numbers, lengths (`px` / `em` / `rem` / …),
//!   percentages, colours (hex + 20 named), keywords,
//!   strings, and generic function calls (`rgb(...)` etc.
//!   — the parser does not interpret the body, but
//!   records the structure)
//!
//! Not in this chunk (deferred to M5+):
//!
//! - The CSS cascade (priority / origin ordering)
//! - `var(--x)` and `calc(...)` resolution
//! - Selector matching against a DOM (the matcher
//!   consumes the parsed `Stylesheet` and lives in
//!   `spiral-gyre`)
//! - Shorthand expansion (`margin: 1px 2px 3px 4px`)
//! - The full CSS @-rule interpretation (`@media` query
//!   matching, `@keyframes` rule extraction, etc.)
//!
//! Per the architect's Fork 1-B, `spiral-css` becomes a
//! thin `#[deprecated]` re-export shim around this
//! module. The rewire happens in
//! `crates/spiral-css/src/lib.rs`.

pub mod parser;
pub mod selector;
pub mod specificity;
pub mod tokenizer;
pub mod value;

// Re-exports the public CSS types. They appear "unused" to
// rustc inside `spiral-fmt` itself — `spiral-css` is the
// downstream consumer.
#[allow(unused_imports)]
pub use parser::{
    parse as parse_stylesheet, AtBlock, AtRule, Declaration, QualifiedRule, Rule, Stylesheet,
};
#[allow(unused_imports)]
pub use selector::{
    AttributeCase, AttributeMatcher, AttributeSelector, Combinator, ComplexSelector,
    ComplexSelectorStep, CompoundSelector, SelectorList, TypeSelector,
};
#[allow(unused_imports)]
pub use specificity::Specificity;
#[allow(unused_imports)]
pub use value::Value;

use crate::error::FormatError;

/// Parse a CSS stylesheet from a source string.
///
/// This is the canonical entry point for callers that
/// want the M4.4.1-minimum CSS surface. It produces a
/// [`Stylesheet`] that the cascade (M5+) and the
/// selector matcher (in `spiral-gyre`) consume.
pub fn parse(source: &str) -> Result<Stylesheet, FormatError> {
    parser::parse(source).map_err(FormatError::Css)
}
