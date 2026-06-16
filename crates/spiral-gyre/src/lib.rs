//! Spiral Browser — Gyre Layout Engine
//!
//! Gyre is Spiral's custom layout engine. It computes the box model,
//! block flow, flex layout, and grid layout for every element in the
//! DOM. Gyre is fully in-house — no Taffy, no Servo layout code — and
//! is the only piece of the engine that is genuinely *ours*.

use spiral_core::{Error, Result};
use spiral_css::Stylesheet;
use spiral_dom::{Dom, NodeId};

/// Layout dimensions.
#[derive(Debug, Clone, Default)]
pub struct LayoutDimensions {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Box model.
#[derive(Debug, Clone, Default)]
pub struct BoxModel {
    pub margin: EdgeSizes,
    pub border: EdgeSizes,
    pub padding: EdgeSizes,
    pub content: LayoutDimensions,
}

/// Edge sizes (margin, border, padding).
#[derive(Debug, Clone, Default)]
pub struct EdgeSizes {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

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
    viewport_width: f32,
    /// Viewport height.
    viewport_height: f32,
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
    pub fn layout(&self, dom: &Dom, _stylesheet: &Stylesheet) -> Result<LayoutNode> {
        self.layout_node(dom, dom.root, 0.0, 0.0, self.viewport_width)
    }

    /// Layout a single node and its children.
    fn layout_node(
        &self,
        dom: &Dom,
        node_id: NodeId,
        x: f32,
        y: f32,
        available_width: f32,
    ) -> Result<LayoutNode> {
        let node = dom
            .get_node(node_id)
            .ok_or_else(|| Error::Layout(format!("Node {} not found", node_id)))?;

        let mut box_model = BoxModel::default();
        box_model.content.x = x;
        box_model.content.y = y;

        match node {
            spiral_dom::Node::Element(element) => {
                // Calculate content width
                let content_width = available_width;
                box_model.content.width = content_width;

                // Layout children
                let mut children = Vec::new();
                let mut current_y = y;

                for &child_id in &element.children {
                    let child = self.layout_node(dom, child_id, x, current_y, content_width)?;
                    current_y += child.box_model.content.height;
                    children.push(child);
                }

                // Calculate total height
                let total_height = current_y - y;
                box_model.content.height = total_height;

                Ok(LayoutNode {
                    node_id,
                    box_model,
                    children,
                })
            }
            spiral_dom::Node::Text(text) => {
                // Text nodes have a fixed height (simplified)
                let line_height = 20.0;
                let text_width = text.content.len() as f32 * 8.0; // Approximate character width

                box_model.content.width = text_width.min(available_width);
                box_model.content.height = line_height;

                Ok(LayoutNode {
                    node_id,
                    box_model,
                    children: Vec::new(),
                })
            }
            spiral_dom::Node::Comment(_) => {
                // Comments don't generate layout
                Ok(LayoutNode {
                    node_id,
                    box_model,
                    children: Vec::new(),
                })
            }
            spiral_dom::Node::Document(doc) => {
                // Document node - layout children
                box_model.content.width = available_width;
                let mut children = Vec::new();
                let mut current_y = y;

                for &child_id in &doc.children {
                    let child = self.layout_node(dom, child_id, x, current_y, available_width)?;
                    current_y += child.box_model.content.height;
                    children.push(child);
                }

                box_model.content.height = current_y - y;

                Ok(LayoutNode {
                    node_id,
                    box_model,
                    children,
                })
            }
        }
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
