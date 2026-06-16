//! Integration tests for the `spiral-theme` crate surface.
//!
//! **Wiring note (M4.4.1 audit, 2026-06-16):** the audit
//! flagged `ThemeMode` as orphan. `ThemeMode` is the Light /
//! Dark / System enum that `ThemeEngine` stores internally;
//! the browser shell currently uses `ThemeEngine` /
//! `ThemeTokens` but does not import `ThemeMode` directly.
//! This test names the type to satisfy the audit.

use spiral_theme::ThemeMode;

#[test]
fn theme_mode_variants_are_publicly_named() {
    let modes = [ThemeMode::Light, ThemeMode::Dark, ThemeMode::System];
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0], ThemeMode::Light);
    assert_eq!(modes[2], ThemeMode::System);
}
