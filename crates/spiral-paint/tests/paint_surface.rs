//! Integration tests for the `spiral-paint` public surface.

use spiral_core::Color;
use spiral_paint::{DisplayList, PaintEngine, RenderOp};

#[test]
fn test_paint_surface_wiring() {
    let _engine = PaintEngine::new();
    let _list = DisplayList { ops: vec![] };
    let _op = RenderOp::FillRect {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
        color: Color {
            r: 0,
            g: 0,
            b: 0,
            a: 1.0,
        },
    };
}
