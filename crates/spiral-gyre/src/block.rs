//! Block layout implementation for the Gyre layout engine.

use crate::box_model::{BoxModel, EdgeSizes, LayoutDimensions};
use crate::style::ResolvedStyle;
use spiral_core::{Error, Result};
use spiral_css::Stylesheet;
use spiral_css::Value;
use spiral_dom::{Dom, Node, NodeId};

/// Absolute resolved geometry values for a layout node.
#[derive(Debug, Clone, Copy)]
pub struct ResolvedBox {
    pub margin_top: f32,
    pub margin_right: f32,
    pub margin_bottom: f32,
    pub margin_left: f32,
    pub border_top: f32,
    pub border_right: f32,
    pub border_bottom: f32,
    pub border_left: f32,
    pub padding_top: f32,
    pub padding_right: f32,
    pub padding_bottom: f32,
    pub padding_left: f32,
    pub width: f32,
    pub height: Option<f32>,
}

/// Resolve float pixel length from CSS Value.
pub fn resolve_length(val: &Value, containing_width: f32) -> f32 {
    match val {
        Value::Length(px) => *px,
        Value::Number(num) => *num,
        Value::Percentage(pct) => (pct / 100.0) * containing_width,
        _ => 0.0,
    }
}

/// Resolve CSS box styling properties to absolute pixel dimensions (CSS 2.1 §10.3.3).
pub fn resolve_box_geometry(style: &ResolvedStyle, containing_width: f32) -> ResolvedBox {
    let border_top = resolve_length(&style.border_top_width, containing_width);
    let border_right = resolve_length(&style.border_right_width, containing_width);
    let border_bottom = resolve_length(&style.border_bottom_width, containing_width);
    let border_left = resolve_length(&style.border_left_width, containing_width);

    let padding_top = resolve_length(&style.padding_top, containing_width);
    let padding_right = resolve_length(&style.padding_right, containing_width);
    let padding_bottom = resolve_length(&style.padding_bottom, containing_width);
    let padding_left = resolve_length(&style.padding_left, containing_width);

    let margin_top = if let Value::Keyword(kw) = &style.margin_top {
        if kw.to_lowercase() == "auto" {
            0.0
        } else {
            resolve_length(&style.margin_top, containing_width)
        }
    } else {
        resolve_length(&style.margin_top, containing_width)
    };

    let margin_bottom = if let Value::Keyword(kw) = &style.margin_bottom {
        if kw.to_lowercase() == "auto" {
            0.0
        } else {
            resolve_length(&style.margin_bottom, containing_width)
        }
    } else {
        resolve_length(&style.margin_bottom, containing_width)
    };

    let w_val = style.width.as_ref();
    let ml_val = &style.margin_left;
    let mr_val = &style.margin_right;

    let ml_is_auto = matches!(ml_val, Value::Keyword(kw) if kw.to_lowercase() == "auto");
    let mr_is_auto = matches!(mr_val, Value::Keyword(kw) if kw.to_lowercase() == "auto");
    let w_is_auto =
        w_val.is_none() || matches!(w_val, Some(Value::Keyword(kw)) if kw.to_lowercase() == "auto");

    let (width, margin_left, margin_right) = if !w_is_auto {
        let width = resolve_length(w_val.unwrap(), containing_width);
        let non_content_w = border_left + padding_left + padding_right + border_right;
        let remaining = containing_width - width - non_content_w;

        let (ml, mr) = if ml_is_auto && mr_is_auto {
            let half = (remaining / 2.0).max(0.0);
            (half, half)
        } else if ml_is_auto {
            let mr = resolve_length(mr_val, containing_width);
            let ml = (remaining - mr).max(0.0);
            (ml, mr)
        } else if mr_is_auto {
            let ml = resolve_length(ml_val, containing_width);
            let mr = (remaining - ml).max(0.0);
            (ml, mr)
        } else {
            let ml = resolve_length(ml_val, containing_width);
            let mr = remaining - ml;
            (ml, mr)
        };
        (width, ml, mr)
    } else {
        let ml = if ml_is_auto {
            0.0
        } else {
            resolve_length(ml_val, containing_width)
        };
        let mr = if mr_is_auto {
            0.0
        } else {
            resolve_length(mr_val, containing_width)
        };
        let non_content_w = ml + border_left + padding_left + padding_right + border_right + mr;
        let width = (containing_width - non_content_w).max(0.0);
        (width, ml, mr)
    };

    let height = style.height.as_ref().and_then(|h| {
        if matches!(h, Value::Keyword(kw) if kw.to_lowercase() == "auto") {
            None
        } else {
            Some(resolve_length(h, containing_width))
        }
    });

    ResolvedBox {
        margin_top,
        margin_right,
        margin_bottom,
        margin_left,
        border_top,
        border_right,
        border_bottom,
        border_left,
        padding_top,
        padding_right,
        padding_bottom,
        padding_left,
        width,
        height,
    }
}

fn get_first_child_margin_top(
    dom: &Dom,
    children: &[NodeId],
    stylesheet: &Stylesheet,
    width: f32,
) -> f32 {
    for &child_id in children {
        if let Some(Node::Element(_)) = dom.get_node(child_id) {
            let child_style = ResolvedStyle::resolve(dom, child_id, stylesheet);
            if child_style.display == "none" {
                continue;
            }
            return if let Value::Keyword(kw) = &child_style.margin_top {
                if kw.to_lowercase() == "auto" {
                    0.0
                } else {
                    resolve_length(&child_style.margin_top, width)
                }
            } else {
                resolve_length(&child_style.margin_top, width)
            };
        }
    }
    0.0
}

/// Recursively computes layout coordinates and dimensions for a DOM Node.
pub fn layout_node(
    dom: &Dom,
    node_id: NodeId,
    stylesheet: &Stylesheet,
    x: f32,
    y: f32,
    containing_width: f32,
) -> Result<crate::LayoutNode> {
    let node = dom
        .get_node(node_id)
        .ok_or_else(|| Error::Layout(format!("Node {} not found", node_id)))?;

    match node {
        Node::Document(doc) => {
            let mut box_model = BoxModel::default();
            box_model.content.x = x;
            box_model.content.y = y;
            box_model.content.width = containing_width;

            let mut children = Vec::new();
            let mut current_y = y;
            let mut last_margin_bottom = 0.0;

            for &child_id in &doc.children {
                let child_style = ResolvedStyle::resolve(dom, child_id, stylesheet);
                if child_style.display == "none" {
                    continue;
                }
                let child_margin_top = if let Value::Keyword(kw) = &child_style.margin_top {
                    if kw.to_lowercase() == "auto" {
                        0.0
                    } else {
                        resolve_length(&child_style.margin_top, containing_width)
                    }
                } else {
                    resolve_length(&child_style.margin_top, containing_width)
                };

                let child_y = current_y + child_margin_top;
                let child = layout_node(dom, child_id, stylesheet, x, child_y, containing_width)?;
                let child_total_height = child.box_model.content.height
                    + child.box_model.padding.bottom
                    + child.box_model.border.bottom;
                current_y = child.box_model.content.y + child_total_height;
                last_margin_bottom = child.box_model.margin.bottom;
                children.push(child);
            }

            box_model.content.height = (current_y - y + last_margin_bottom).max(0.0);

            Ok(crate::LayoutNode {
                node_id,
                box_model,
                children,
            })
        }
        Node::Element(element) => {
            let style = ResolvedStyle::resolve(dom, node_id, stylesheet);

            if style.display == "none" {
                return Ok(crate::LayoutNode {
                    node_id,
                    box_model: BoxModel::default(),
                    children: Vec::new(),
                });
            }

            let geom = resolve_box_geometry(&style, containing_width);

            // First child top margin collapse
            let first_child_margin_top =
                get_first_child_margin_top(dom, &element.children, stylesheet, geom.width);
            let collapsed_margin_top = if geom.padding_top == 0.0 && geom.border_top == 0.0 {
                geom.margin_top.max(first_child_margin_top)
            } else {
                geom.margin_top
            };

            let mut box_model = BoxModel {
                margin: EdgeSizes {
                    top: collapsed_margin_top,
                    right: geom.margin_right,
                    bottom: geom.margin_bottom,
                    left: geom.margin_left,
                },
                border: EdgeSizes {
                    top: geom.border_top,
                    right: geom.border_right,
                    bottom: geom.border_bottom,
                    left: geom.border_left,
                },
                padding: EdgeSizes {
                    top: geom.padding_top,
                    right: geom.padding_right,
                    bottom: geom.padding_bottom,
                    left: geom.padding_left,
                },
                content: LayoutDimensions {
                    x: x + geom.margin_left + geom.border_left + geom.padding_left,
                    y: 0.0,
                    width: geom.width,
                    height: 0.0,
                },
            };

            let shift_top = collapsed_margin_top - geom.margin_top;
            let parent_border_box_y = y + shift_top;
            let parent_content_y = parent_border_box_y + geom.border_top + geom.padding_top;
            let mut current_y = parent_content_y;

            if !(geom.padding_top == 0.0 && geom.border_top == 0.0) {
                current_y += first_child_margin_top;
            }

            let mut children = Vec::new();
            let mut prev_margin_bottom: f32 = 0.0;

            for (idx, &child_id) in element.children.iter().enumerate() {
                let child_style = ResolvedStyle::resolve(dom, child_id, stylesheet);
                if child_style.display == "none" {
                    continue;
                }

                let child_margin_top = if let Value::Keyword(kw) = &child_style.margin_top {
                    if kw.to_lowercase() == "auto" {
                        0.0
                    } else {
                        resolve_length(&child_style.margin_top, geom.width)
                    }
                } else {
                    resolve_length(&child_style.margin_top, geom.width)
                };

                if idx > 0 {
                    let collapsed_sibling = prev_margin_bottom.max(child_margin_top);
                    current_y += collapsed_sibling;
                }

                let child_node = layout_node(
                    dom,
                    child_id,
                    stylesheet,
                    box_model.content.x,
                    current_y,
                    geom.width,
                )?;

                current_y = child_node.box_model.content.y
                    + child_node.box_model.content.height
                    + child_node.box_model.padding.bottom
                    + child_node.box_model.border.bottom;

                prev_margin_bottom = child_node.box_model.margin.bottom;
                children.push(child_node);
            }

            let content_height = if let Some(h) = geom.height {
                h
            } else {
                let mut height_val = (current_y - parent_content_y).max(0.0);
                if geom.padding_bottom == 0.0 && geom.border_bottom == 0.0 {
                    box_model.margin.bottom = box_model.margin.bottom.max(prev_margin_bottom);
                } else {
                    height_val += prev_margin_bottom;
                }
                height_val
            };

            box_model.content.height = content_height;
            box_model.content.y = parent_content_y;

            Ok(crate::LayoutNode {
                node_id,
                box_model,
                children,
            })
        }
        Node::Text(text) => {
            let line_height = 20.0;
            let text_width = text.content.len() as f32 * 8.0;

            let mut box_model = BoxModel::default();
            box_model.content.x = x;
            box_model.content.y = y;
            box_model.content.width = text_width.min(containing_width);
            box_model.content.height = line_height;

            Ok(crate::LayoutNode {
                node_id,
                box_model,
                children: Vec::new(),
            })
        }
        Node::Comment(_) => Ok(crate::LayoutNode {
            node_id,
            box_model: BoxModel::default(),
            children: Vec::new(),
        }),
    }
}
