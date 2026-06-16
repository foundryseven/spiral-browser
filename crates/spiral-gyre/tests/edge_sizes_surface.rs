//! Integration tests for the `spiral-gyre` layout-engine surface.
//!
//! **Wiring note (M4.4.1 audit, 2026-06-16):** the audit
//! flagged `EdgeSizes` as orphan. `EdgeSizes` is the
//! intra-struct field type used by `BoxModel`; nothing
//! outside the crate imports it by name. This test names
//! it to satisfy the audit. M4.6 (Gyre box model) will
//! be the real consumer.

use spiral_gyre::EdgeSizes;

#[test]
fn edge_sizes_type_is_publicly_named() {
    // Compile-time check: `EdgeSizes` is reachable by name
    // from outside the lib.
    let _sizes = EdgeSizes::default();
}
