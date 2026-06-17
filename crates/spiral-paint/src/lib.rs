//! Spiral Browser — Paint Engine
//!
//! Display list construction for the Spiral Browser.

pub use spiral_core::Color;
use spiral_core::Result;
use spiral_gyre::LayoutNode;

/// Render operations for the display list.
#[derive(Debug, Clone)]
pub enum RenderOp {
    /// Fill a rectangle with a color.
    FillRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
    },
    /// Stroke a rectangle with a color and width.
    StrokeRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        stroke_width: f32,
    },
    /// Draw text at a position.
    DrawText {
        x: f32,
        y: f32,
        text: String,
        font_size: f32,
        color: Color,
    },
    /// Draw an image.
    DrawImage {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        data: Vec<u8>,
        img_width: u32,
        img_height: u32,
    },
    /// Clip to a rectangle.
    Clip {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        ops: Vec<RenderOp>,
    },
    /// Apply a transform.
    Transform {
        matrix: [f32; 6],
        ops: Vec<RenderOp>,
    },
    /// Push a new layer with opacity.
    PushLayer { opacity: f32 },
    /// Pop the current layer.
    PopLayer,
}

/// Display list.
#[derive(Debug, Clone)]
pub struct DisplayList {
    pub ops: Vec<RenderOp>,
}

/// Paint engine that builds display lists from layout trees.
pub struct PaintEngine;

impl PaintEngine {
    /// Create a new paint engine.
    pub fn new() -> Self {
        Self
    }

    /// Build a display list from a layout tree.
    pub fn paint(&self, layout: &LayoutNode) -> Result<DisplayList> {
        let mut ops = Vec::new();
        self.paint_node(layout, &mut ops)?;
        Ok(DisplayList { ops })
    }

    /// Paint a single layout node.
    fn paint_node(&self, node: &LayoutNode, ops: &mut Vec<RenderOp>) -> Result<()> {
        let bm = &node.box_model;

        // Draw background (simplified - white for now)
        if bm.content.width > 0.0 && bm.content.height > 0.0 {
            ops.push(RenderOp::FillRect {
                x: bm.content.x,
                y: bm.content.y,
                width: bm.content.width,
                height: bm.content.height,
                color: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 1.0,
                },
            });
        }

        // Paint children
        for child in &node.children {
            self.paint_node(child, ops)?;
        }

        Ok(())
    }
}

impl Default for PaintEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spiral_gyre::BoxModel;

    #[test]
    fn test_paint_empty_layout() {
        let engine = PaintEngine::new();
        let layout = LayoutNode {
            node_id: 0,
            box_model: BoxModel::default(),
            children: Vec::new(),
        };
        let display_list = engine.paint(&layout).unwrap();
        assert!(display_list.ops.is_empty());
    }

    #[test]
    fn test_paint_with_content() {
        let engine = PaintEngine::new();
        let layout = LayoutNode {
            node_id: 0,
            box_model: BoxModel {
                content: spiral_gyre::LayoutDimensions {
                    x: 0.0,
                    y: 0.0,
                    width: 100.0,
                    height: 100.0,
                },
                ..Default::default()
            },
            children: Vec::new(),
        };
        let display_list = engine.paint(&layout).unwrap();
        assert_eq!(display_list.ops.len(), 1);
    }
}
