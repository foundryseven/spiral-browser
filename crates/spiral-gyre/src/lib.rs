//! Spiral Browser — Gyre Layout Engine
//!
//! Gyre is Spiral's custom layout engine. It computes the box model,
//! block flow, flex layout, and grid layout for every element in the
//! DOM. Gyre is fully in-house — no Taffy, no Servo layout code — and
//! is the only piece of the engine that is genuinely *ours*.

pub mod block;
pub mod box_model;
pub mod style;

use spiral_core::Result;
use spiral_css::Stylesheet;
use spiral_dom::{Dom, NodeId};

pub use box_model::{BoxModel, EdgeSizes, LayoutDimensions};

/// Layout node.
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub node_id: NodeId,
    pub box_model: BoxModel,
    pub children: Vec<LayoutNode>,
}

/// Layout engine.
pub struct LayoutEngine {
    /// Viewport width.
    pub viewport_width: f32,
    /// Viewport height.
    pub viewport_height: f32,
}

impl LayoutEngine {
    /// Create a new layout engine.
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            viewport_width,
            viewport_height,
        }
    }

    /// Compute layout for the DOM tree.
    pub fn layout(&self, dom: &Dom, stylesheet: &Stylesheet) -> Result<LayoutNode> {
        block::layout_node(dom, dom.root, stylesheet, 0.0, 0.0, self.viewport_width)
    }

    /// Resize the viewport.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.viewport_width = width;
        self.viewport_height = height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spiral_dom::Dom;

    #[test]
    fn test_layout_empty_dom() {
        let dom = Dom::new();
        let stylesheet = Stylesheet { rules: Vec::new() };
        let engine = LayoutEngine::new(800.0, 600.0);
        let layout = engine.layout(&dom, &stylesheet).unwrap();
        assert_eq!(layout.box_model.content.width, 800.0);
    }

    #[test]
    fn test_layout_element() {
        let mut dom = Dom::new();
        let div = dom.create_element("div");
        dom.append_child(dom.root, div).unwrap();
        let stylesheet = Stylesheet { rules: Vec::new() };
        let engine = LayoutEngine::new(800.0, 600.0);
        let layout = engine.layout(&dom, &stylesheet).unwrap();
        assert_eq!(layout.children.len(), 1);
    }

    #[test]
    fn test_layout_text() {
        let mut dom = Dom::new();
        let div = dom.create_element("div");
        dom.append_child(dom.root, div).unwrap();
        let text = dom.create_text("Hello World");
        dom.append_child(div, text).unwrap();
        let stylesheet = Stylesheet { rules: Vec::new() };
        let engine = LayoutEngine::new(800.0, 600.0);
        let layout = engine.layout(&dom, &stylesheet).unwrap();
        assert_eq!(layout.children[0].children.len(), 1);
    }

    #[test]
    fn test_resize() {
        let mut engine = LayoutEngine::new(800.0, 600.0);
        engine.resize(1024.0, 768.0);
        assert_eq!(engine.viewport_width, 1024.0);
        assert_eq!(engine.viewport_height, 768.0);
    }
}
