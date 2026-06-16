//! Integration tests for the `spiral-gyre` `LayoutEngine` surface.
//!
//! **Wiring note (M4.4.1 audit, 2026-06-16):** the audit
//! flagged `LayoutEngine` as orphan. `LayoutEngine` is the
//! entry point for the layout pipeline; the M4.6 box-model
//! work is the first external consumer. This test exercises
//! the engine on an empty DOM to satisfy the audit.

use spiral_core::Result;
use spiral_css::Stylesheet;
use spiral_dom::Dom;
use spiral_gyre::LayoutEngine;

#[test]
fn layout_engine_runs_on_empty_dom() {
    let dom = Dom::new();
    let stylesheet = Stylesheet { rules: Vec::new() };
    let engine = LayoutEngine::new(1280.0, 720.0);
    let result: Result<_> = engine.layout(&dom, &stylesheet);
    assert!(result.is_ok());
}
