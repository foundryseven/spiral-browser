//! Spiral's vendored parser crate.
//!
//! `spiral-fmt` provides the format-parsing surface for the
//! engine: HTML and CSS. The implementation is from-spec —
//! every component is written against the WHATWG / W3C
//! standards, with no verbatim or near-verbatim copying from
//! any external source. This is in keeping with the M4 audit
//! posture and the user's 2026-06-15 mandate that "our tech
//! where it matters" includes the parser layer.
//!
//! ## Sub-modules
//!
//! - [`html`] — HTML5 tokeniser + tree builder. Public entry
//!   point is [`parse_html`].
//! - [`css`] — CSS Syntax 3 tokeniser + parser. Public entry
//!   point is [`parse_css`].
//! - [`error`] — [`FormatError`].
//!
//! ## Scope
//!
//! M4.4.1 is the minimum-viable parser that unblocks the rest
//! of the engine. See the module-level docs for the precise
//! limitations. The 80 additional WPT cases the stretch target
//! requires land in M4.4.2 (Chunk 4 of the gap-analysis plan).

#![allow(clippy::result_large_err)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]

pub mod cursor;
pub mod error;
pub mod html;
pub mod token;

pub use error::FormatError;

/// Parse `source` as HTML5 and return a `spiral_dom::Dom`.
///
/// This is a thin alias for [`html::parse`]. The function takes
/// a `&str` and returns a `Result<Dom, FormatError>`. The
/// minimum-viable parser is lenient: malformed input is
/// recovered from rather than rejected.
///
/// See [`html`] for the implementation and the M4.4.1 scope.
pub fn parse_html(source: &str) -> Result<spiral_dom::Dom, FormatError> {
    html::parse(source)
}

/// Parse `source` as a CSS stylesheet.
///
/// M4.4.1 ships a from-spec CSS Syntax Level 3 parser. The
/// surface is the M4.4.1 minimum — selectors with type,
/// class, ID, attribute, pseudo-class, pseudo-element, and
/// all four combinators; specificity per Selectors Level 4;
/// at-rule recognition (block-form and semicolon-form);
/// colour, length, percentage, keyword, string, and
/// function values. The full cascade and `var()` / `calc()`
/// resolution are M5+.
///
/// See [`css`] for the implementation and the precise scope.
pub fn parse_css(source: &str) -> Result<css::Stylesheet, FormatError> {
    css::parse(source)
}

// CSS public surface re-exported at the crate root for
// downstream consumers (`spiral-css`, `spiral-gyre`, e2e
// tests). The `css` module itself stays `mod css` (private
// to `spiral-fmt`) to keep the module layout in one place
// while still exposing the types callers need.
#[allow(unused_imports)]
pub use css::{
    parse as parse_stylesheet, AtBlock, AtRule, AttributeCase, AttributeMatcher, AttributeSelector,
    Combinator, ComplexSelector, ComplexSelectorStep, CompoundSelector, Declaration, QualifiedRule,
    Rule, SelectorList, Specificity, Stylesheet, TypeSelector, Value,
};

mod css;
