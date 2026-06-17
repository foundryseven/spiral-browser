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

/// The output of [`parse_html_fragment`]: a self-contained DOM
/// plus the top-level node IDs that the fragment algorithm
/// produced.
///
/// WHATWG HTML §12.4 "HTML fragment parsing algorithm" returns a
/// list of nodes. In Spiral, those nodes live in a small DOM
/// that the caller can either inspect in place or transplant
/// into another DOM.
///
/// The DOM is owned by the Fragment — dropping the Fragment
/// drops the DOM and its nodes.
pub struct Fragment {
    /// The DOM containing the implicit `<html><head><body>`
    /// wrappers and the parsed nodes. Owned by the Fragment.
    pub dom: spiral_dom::Dom,
    /// Top-level parsed nodes, in source order.
    ///
    /// For most contexts these are the children of the synthetic
    /// body / context element. The caller can read their tags
    /// and attributes via `dom.get_tag(id)` / `dom.get_attributes(id)`,
    /// or move them into another DOM with `dom.append_child`.
    pub nodes: Vec<spiral_dom::NodeId>,
}

/// Parse an HTML fragment using the WHATWG HTML §12.4 algorithm.
///
/// `context` is the DOM that supplies the context element. The
/// fragment algorithm reads `context_id`'s tag name to decide
/// which insertion mode to start with (see the table in the
/// module docs of [`html::fragment`]).
///
/// The returned [`Fragment`] owns its own DOM, separate from
/// `context`. The caller can either keep them separate (e.g. to
/// validate the fragment before insertion) or call
/// `frag.dom.append_child(parent, id)` to transplant each node
/// into the caller's DOM.
///
/// Required for `Element.innerHTML = "..."`, the `<template>`
/// element's content document fragment, and the Vortex
/// `Element.innerHTML` JS binding.
pub fn parse_html_fragment(
    context: &spiral_dom::Dom,
    context_id: spiral_dom::NodeId,
    source: &str,
) -> Result<Fragment, FormatError> {
    html::fragment::parse(context, context_id, source)
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
