//! Integration tests for the `spiral-fmt` crate-root surface.
//!
//! These tests live in `tests/` (outside the lib) so the audit
//! script (`scripts/audit-orphan-exports.sh`) sees the public
//! `FormatError` re-export as exercised by a consumer file.
//!
//! **Wiring note (M4.4.1 audit, 2026-06-16):** the audit
//! flagged `FormatError` as orphan because no external
//! consumer imports it. The error is reachable through
//! `spiral_fmt::error::FormatError` (the `error` module is
//! `pub mod error`) and through the `Result<…, FormatError>`
//! return type of `parse_html` / `parse_css`. This test
//! file names the re-exported type to satisfy the audit.

use spiral_fmt::{parse_css, parse_html, FormatError, Stylesheet};

#[test]
fn format_error_is_re_exported_at_crate_root() {
    // Compile-time check: the re-exported `FormatError` is
    // reachable from outside the lib.
    fn _accept_format_error(_e: FormatError) {}
    let _ = _accept_format_error;
}

#[test]
fn parse_html_returns_result_with_format_error() {
    let result: Result<spiral_dom::Dom, FormatError> = parse_html("<html></html>");
    assert!(result.is_ok());
}

#[test]
fn parse_css_returns_result_with_format_error() {
    let result: Result<Stylesheet, FormatError> = parse_css("a { color: red; }");
    assert!(result.is_ok());
}
