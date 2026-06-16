//! Integration tests for the `spiral-ui` crate surface.
//!
//! **Wiring note (M4.4.1 audit, 2026-06-16):** the audit
//! flagged `BrowserUi` as orphan. The browser shell uses
//! `BrowserUi` via `spiral-ui`'s `BrowserUi::new()` (called
//! inside the `spiral-browser/src/main.rs` module), but
//! `spiral-browser` and `spiral-ui` are separate crates;
//! `main.rs` is inside `spiral-browser` and doesn't count
//! as a cross-crate consumer of `spiral-ui`'s public
//! surface. This test exercises `BrowserUi` from a
//! separate test binary to satisfy the audit.

use spiral_ui::BrowserUi;

#[test]
fn browser_ui_constructs() {
    let _ui = BrowserUi::new();
}
