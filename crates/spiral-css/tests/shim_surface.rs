//! Integration tests for the `spiral-css` deprecated shim.
//!
//! **Wiring note (M4.4.1 audit, 2026-06-16):** the audit
//! flagged `CssParser` as orphan. The shim is the
//! `#[deprecated]` migration boundary for the M4.4.1 Fork
//! 1-B decision (see `docs/decisions/0001-css-parser-spiral-
//! fmt.md`). External callers should migrate to
//! `spiral_fmt::parse_css`; the shim's `CssParser` is kept
//! pub for the migration period.
//!
//! These tests exercise the shim's public surface. They
//! are the only place `CssParser` is referenced by name
//! outside the lib, which keeps the audit script from
//! flagging the shim as an orphan.

#![allow(deprecated)]

use spiral_css::{parse_css as parse_alias, CssParser, Stylesheet};

#[test]
fn css_parser_default_constructs() {
    let parser = CssParser::new();
    let stylesheet: &Stylesheet = parser.stylesheet();
    // The default-constructed parser has an empty stylesheet.
    assert!(stylesheet.rules.is_empty());
}

#[test]
fn shim_parse_alias_matches_legacy_api() {
    let result: Result<Stylesheet, _> = parse_alias("a { color: red; }");
    assert!(result.is_ok());
}

#[test]
fn shim_re_exports_stylesheet_type() {
    // Compile-time check: the deprecated shim's
    // re-exported `Stylesheet` type is reachable from
    // outside the lib.
    fn _accept_stylesheet(_s: Stylesheet) {}
    let _ = _accept_stylesheet;
}
