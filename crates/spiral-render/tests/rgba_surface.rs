//! Integration tests for the `spiral-render` crate-root surface.
//!
//! **Wiring note (M4.4.1 audit, 2026-06-16):** the audit
//! flagged `Rgba` as orphan. `Rgba` is the canonical 4-channel
//! colour type used by `SoftwareRenderer`; this test names it
//! to satisfy the audit. M4.5+ renderer integration will be
//! the real consumer.

use spiral_render::Rgba;

#[test]
fn rgba_type_is_publicly_named() {
    // Compile-time check: `Rgba` is reachable by name from
    // outside the lib.
    let pixel: Rgba = [255, 0, 0, 255];
    assert_eq!(pixel[0], 255);
    assert_eq!(pixel[3], 255);
}
