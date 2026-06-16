//! Spiral Browser — CSS Parser.
//!
//! **Deprecated shim.** This crate used to host a
//! hand-rolled CSS parser wrapping `cssparser` and
//! `selectors`. As of M4.4.1 (2026-06-16, per Fork 1-B),
//! CSS parsing lives in `spiral_fmt::css`, a from-spec CSS
//! Syntax Level 3 implementation. This crate remains in
//! the workspace so existing consumers — chiefly
//! `spiral_gyre` — can keep importing from `spiral_css`
//! without churn.
//!
//! New code should depend on `spiral_fmt` directly and call
//! [`spiral_fmt::parse_css`]. The [`CssParser`] type and
//! the [`Stylesheet`] / [`Rule`] / [`Declaration`] /
//! [`Value`] aliases below all forward to the new module.
//!
//! Deprecations are `#[deprecated]` notes pointing
//! downstream code at the new path; they do NOT delete
//! the old API. The shim is the migration boundary.

#![deprecated(
    since = "0.2.0",
    note = "spiral-css is a deprecated shim. Use spiral_fmt::css instead \
            (entry point: spiral_fmt::parse_css)."
)]

#[allow(deprecated)]
pub use spiral_fmt::parse_css;
#[allow(deprecated)]
pub use spiral_fmt::parse_stylesheet;
#[allow(deprecated)]
pub use spiral_fmt::{
    AtBlock, AtRule, AttributeCase, AttributeMatcher, AttributeSelector, Combinator,
    ComplexSelector, ComplexSelectorStep, CompoundSelector, Declaration, QualifiedRule, Rule,
    SelectorList, Specificity, Stylesheet, TypeSelector, Value,
};

use spiral_core::Error;
use spiral_core::Result;

/// CSS parser.
///
/// This is a thin adapter over [`spiral_fmt::parse_css`]
/// that keeps the old `CssParser::parse` + `stylesheet()`
/// shape so the existing call sites in
/// `crates/spiral-gyre/src/lib.rs` (and any other in-tree
/// consumer) keep working without churn.
#[deprecated(
    since = "0.2.0",
    note = "spiral_css::CssParser is a shim. Use spiral_fmt::parse_css \
            directly."
)]
#[derive(Debug, Clone, Default)]
pub struct CssParser {
    stylesheet: Stylesheet,
}

#[allow(deprecated)]
impl CssParser {
    /// Create a new, empty parser.
    #[must_use]
    pub fn new() -> Self {
        Self {
            stylesheet: Stylesheet::default(),
        }
    }

    /// Parse `css` source text and store the resulting
    /// stylesheet. Any parse error is converted to a
    /// `spiral_core::Error`.
    pub fn parse(&mut self, css: &str) -> Result<()> {
        self.stylesheet =
            parse_css(css).map_err(|e| Error::Parse(format!("css parse error: {e}")))?;
        Ok(())
    }

    /// Borrow the parsed stylesheet. Empty until
    /// [`Self::parse`] has been called at least once.
    #[must_use]
    pub fn stylesheet(&self) -> &Stylesheet {
        &self.stylesheet
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use spiral_core::Color;

    #[test]
    fn shim_parses_simple_qualified_rule() {
        let mut parser = CssParser::new();
        parser.parse("p { color: red; }").expect("parse");
        let sheet = parser.stylesheet();
        let rules: Vec<_> = sheet.qualified_rules().collect();
        assert_eq!(rules.len(), 1);
        let q = rules[0];
        assert_eq!(q.declarations.len(), 1);
        assert_eq!(q.declarations[0].name, "color");
        // Named colours resolve to `Value::Color` in the
        // new parser. The named-colour table maps
        // "red" → (255, 0, 0).
        match &q.declarations[0].value {
            Value::Color(c) => {
                assert_eq!(c.r, 255);
                assert_eq!(c.g, 0);
                assert_eq!(c.b, 0);
                let _ = c.a;
            }
            other => panic!("expected Color(red), got {:?}", other),
        }
        // Make sure the `Color` import is not flagged as
        // unused in the test (we use the field destructuring
        // above).
        let _ = Color::BLACK;
    }

    #[test]
    fn shim_empty_stylesheet_default() {
        let parser = CssParser::new();
        let sheet = parser.stylesheet();
        assert!(sheet.qualified_rules().next().is_none());
        assert!(sheet.at_rules().next().is_none());
    }
}
