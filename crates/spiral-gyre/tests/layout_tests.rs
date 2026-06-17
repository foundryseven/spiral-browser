#![allow(deprecated)]

//! Integration tests for crates/spiral-gyre.

use spiral_css::CssParser;
use spiral_dom::Dom;
use spiral_gyre::{LayoutEngine, LayoutNode};

/// Helper to perform layout with a DOM and a CSS stylesheet.
fn layout_dom_css(dom: &Dom, css: &str, width: f32, height: f32) -> LayoutNode {
    let mut parser = CssParser::new();
    parser.parse(css).expect("CSS parse success");
    let engine = LayoutEngine::new(width, height);
    engine
        .layout(dom, parser.stylesheet())
        .expect("Layout success")
}

#[test]
fn test_style_resolution_cascade() {
    let mut dom = Dom::new();
    let div = dom.create_element("div");
    dom.append_child(dom.root, div).unwrap();
    dom.set_attribute(div, "id", "main").unwrap();
    dom.set_attribute(div, "class", "box primary").unwrap();

    // Specificity tests:
    // 1. Tag selector: width: 100px
    // 2. Class selector: width: 200px (wins over tag)
    // 3. ID selector: width: 300px (wins over class)
    // 4. Class selector with !important: width: 400px (wins over ID)
    let css = "
        div { width: 100px; height: 10px; }
        .box { width: 200px; }
        #main { width: 300px; height: 30px; }
        .primary { width: 400px !important; }
    ";

    let layout = layout_dom_css(&dom, css, 1000.0, 800.0);
    let child_layout = &layout.children[0];

    // Width should be 400px because .primary is !important
    assert_eq!(child_layout.box_model.content.width, 400.0);
    // Height should be 30px because #main wins over div
    assert_eq!(child_layout.box_model.content.height, 30.0);
}

#[test]
fn test_selector_matching_combinators() {
    let mut dom = Dom::new();
    let parent = dom.create_element("div");
    dom.append_child(dom.root, parent).unwrap();
    dom.set_attribute(parent, "class", "parent").unwrap();

    let child = dom.create_element("span");
    dom.append_child(parent, child).unwrap();
    dom.set_attribute(child, "class", "child").unwrap();

    let grandchild = dom.create_element("p");
    dom.append_child(child, grandchild).unwrap();

    // 1. Child combinator: .parent > span
    // 2. Descendant combinator: .parent p
    // 3. Invalid child combinator: .parent > p (grandchild is not a direct child of parent)
    let css = "
        .parent > span { width: 100px; }
        .parent p { height: 50px; }
        .parent > p { height: 100px; }
    ";

    let layout = layout_dom_css(&dom, css, 1000.0, 800.0);
    let parent_node = &layout.children[0];
    let child_node = &parent_node.children[0];
    let grandchild_node = &child_node.children[0];

    assert_eq!(child_node.box_model.content.width, 100.0);
    assert_eq!(grandchild_node.box_model.content.height, 50.0);
}

#[test]
fn test_layout_box_geometry_resolution() {
    let mut dom = Dom::new();
    let div = dom.create_element("div");
    dom.append_child(dom.root, div).unwrap();

    // Padding, border, margin shorthands:
    // margin: 10px 20px -> top/bottom 10px, left/right 20px
    // padding: 5px -> all sides 5px
    // border-width: 2px 4px 6px 8px -> top 2px, right 4px, bottom 6px, left 8px
    let css = "
        div {
            width: 500px;
            height: 100px;
            margin: 10px 20px;
            padding: 5px;
            border-width: 2px 4px 6px 8px;
        }
    ";

    let layout = layout_dom_css(&dom, css, 1000.0, 800.0);
    let node = &layout.children[0];

    // Margin assertions
    assert_eq!(node.box_model.margin.top, 10.0);
    assert_eq!(node.box_model.margin.right, 458.0);
    assert_eq!(node.box_model.margin.bottom, 10.0);
    assert_eq!(node.box_model.margin.left, 20.0);

    // Padding assertions
    assert_eq!(node.box_model.padding.top, 5.0);
    assert_eq!(node.box_model.padding.right, 5.0);
    assert_eq!(node.box_model.padding.bottom, 5.0);
    assert_eq!(node.box_model.padding.left, 5.0);

    // Border assertions
    assert_eq!(node.box_model.border.top, 2.0);
    assert_eq!(node.box_model.border.right, 4.0);
    assert_eq!(node.box_model.border.bottom, 6.0);
    assert_eq!(node.box_model.border.left, 8.0);

    // Content box dimensions
    assert_eq!(node.box_model.content.width, 500.0);
    assert_eq!(node.box_model.content.height, 100.0);

    // absolute content position: x should be parent_x + margin_left + border_left + padding_left
    // parent_x is 0 (since it's root), so content.x = 0 + 20 + 8 + 5 = 33.0
    assert_eq!(node.box_model.content.x, 33.0);
}

#[test]
fn test_layout_margin_collapse_siblings() {
    let mut dom = Dom::new();
    let container = dom.create_element("div");
    dom.append_child(dom.root, container).unwrap();
    dom.set_attribute(container, "class", "container").unwrap();

    let sibling1 = dom.create_element("div");
    dom.append_child(container, sibling1).unwrap();
    dom.set_attribute(sibling1, "class", "s1").unwrap();

    let sibling2 = dom.create_element("div");
    dom.append_child(container, sibling2).unwrap();
    dom.set_attribute(sibling2, "class", "s2").unwrap();

    // sibling1 margin-bottom = 30px, sibling2 margin-top = 20px
    // Collapsed sibling margin should be max(30, 20) = 30px.
    // Since container has padding: 10px, the first child's top margin does not collapse with parent.
    let css = "
        .container { padding: 10px; width: 800px; }
        .s1 { height: 50px; margin-bottom: 30px; margin-top: 15px; }
        .s2 { height: 50px; margin-top: 20px; margin-bottom: 15px; }
    ";

    let layout = layout_dom_css(&dom, css, 1000.0, 800.0);
    let container_node = &layout.children[0];
    let s1_node = &container_node.children[0];
    let s2_node = &container_node.children[1];

    // Container's content y starts at margin_top (0) + padding_top (10) = 10.0
    assert_eq!(container_node.box_model.content.y, 10.0);

    // s1 top margin is 15px, so s1 content.y is container_content_y + 15 = 25.0
    assert_eq!(s1_node.box_model.content.y, 25.0);

    // s1 bottom border edge is s1_y + s1_height (50) = 75.0
    // Collapsed margin is max(s1_margin_bottom (30), s2_margin_top (20)) = 30px
    // s2 top border starts at 75.0 + 30 = 105.0. Since s2 has no border/padding, content.y = 105.0
    assert_eq!(s2_node.box_model.content.y, 105.0);

    // s2 bottom border edge is 105.0 + 50 = 155.0
    // Since container bottom padding is 10px (no collapse), container height is (155.0 - 10.0) + s2_margin_bottom (15) = 160.0
    assert_eq!(container_node.box_model.content.height, 160.0);
}

#[test]
fn test_layout_margin_collapse_parent_child() {
    let mut dom = Dom::new();
    let container = dom.create_element("div");
    dom.append_child(dom.root, container).unwrap();
    dom.set_attribute(container, "class", "container").unwrap();

    let child = dom.create_element("div");
    dom.append_child(container, child).unwrap();
    dom.set_attribute(child, "class", "child").unwrap();

    // container has no padding/border, so its margin collapses with child's margin.
    // parent margin-top: 10px, child margin-top: 25px
    // Collapsed margin-top = max(10, 25) = 25px.
    let css = "
        .container { margin-top: 10px; margin-bottom: 5px; width: 800px; }
        .child { height: 100px; margin-top: 25px; margin-bottom: 20px; }
    ";

    let layout = layout_dom_css(&dom, css, 1000.0, 800.0);
    let container_node = &layout.children[0];
    let child_node = &container_node.children[0];

    // Parent container collapsed margin-top should be 25px
    assert_eq!(container_node.box_model.margin.top, 25.0);
    // Parent content.y starts at y (0) + margin_top (25) = 25.0
    assert_eq!(container_node.box_model.content.y, 25.0);

    // Since first child collapsed, child top border starts exactly at parent content y = 25.0
    assert_eq!(child_node.box_model.content.y, 25.0);

    // Parent container collapsed margin-bottom should be max(5, 20) = 20px
    assert_eq!(container_node.box_model.margin.bottom, 20.0);
}

#[test]
fn test_layout_auto_margins_centering() {
    let mut dom = Dom::new();
    let div = dom.create_element("div");
    dom.append_child(dom.root, div).unwrap();

    // centering width: 600px, containing width: 1000px
    // remaining space: 400px -> auto margins split to 200px each
    let css = "
        div {
            width: 600px;
            height: 100px;
            margin-left: auto;
            margin-right: auto;
        }
    ";

    let layout = layout_dom_css(&dom, css, 1000.0, 800.0);
    let node = &layout.children[0];

    assert_eq!(node.box_model.margin.left, 200.0);
    assert_eq!(node.box_model.margin.right, 200.0);
    assert_eq!(node.box_model.content.width, 600.0);
    assert_eq!(node.box_model.content.x, 200.0);
}
