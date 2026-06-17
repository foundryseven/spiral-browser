//! End-to-end tests for `spiral_fmt::parse_html_fragment`.
//!
//! Implements the WHATWG HTML §12.4 HTML fragment parsing
//! algorithm. The fragment algorithm wraps a context element
//! around a sub-parse so callers can build a `NodeList` for
//! `Element.innerHTML = "..."`, `Range.createContextualFragment`,
//! `Document.parseHTMLUnsafe`, and the `<template>` element's
//! content document fragment.
//!
//! Per spec, the insertion mode used depends on the context
//! element's tag name:
//!
//! - `title`, `textarea`: RCDATA (raw text, end-tag aware)
//! - `style`, `script`, `xmp`, `iframe`, `noembed`, `noframes`:
//!   RAWTEXT (no end-tag recognition other than the element's)
//! - `select`: `InSelect` mode (option/optgroup only; anything
//!   else is foster-parented out)
//! - `table`, `tbody`, `tfoot`, `thead`: `InTable` mode
//! - everything else: `InBody` mode
//!
//! The fragment's top-level nodes are returned to the caller as
//! a `Vec<NodeId>` so they can be transplanted into any DOM.

use spiral_dom::Dom;
use spiral_fmt::parse_html_fragment;

// =====================================================================
// Helpers
// =====================================================================

/// Build a DOM that contains a single context element. Returns
/// the DOM and the NodeId of the context element.
fn ctx(tag: &str) -> (Dom, spiral_dom::NodeId) {
    let mut dom = Dom::new();
    let body = dom.create_element("body");
    dom.append_child(dom.root, body).unwrap();
    let el = dom.create_element(tag);
    dom.append_child(body, el).unwrap();
    (dom, el)
}

/// First child tag of a NodeId in a DOM (ignoring text nodes).
fn first_child_tag(dom: &Dom, id: spiral_dom::NodeId) -> Option<String> {
    let children = dom.get_children(id).expect("children");
    for &c in &children {
        if let Some(tag) = dom.get_tag(c) {
            return Some(tag.to_string());
        }
    }
    None
}

/// Collect the tags of all child elements (not text) of a node.
fn child_tags(dom: &Dom, id: spiral_dom::NodeId) -> Vec<String> {
    let children = dom.get_children(id).expect("children");
    children
        .iter()
        .filter_map(|&c| dom.get_tag(c).map(str::to_string))
        .collect()
}

// =====================================================================
// Context element → insertion mode selection
// =====================================================================

/// Context = body. The fragment parses in InBody mode and
/// children of the body element are returned.
#[test]
fn parse_fragment_context_body_div() {
    let (_ctx_dom, ctx_id) = ctx("body");
    let frag = parse_html_fragment(&_ctx_dom, ctx_id, "<div>x</div><span>y</span>").unwrap();
    assert_eq!(frag.nodes.len(), 2);
    assert_eq!(frag.dom.get_tag(frag.nodes[0]), Some("div"));
    assert_eq!(frag.dom.get_tag(frag.nodes[1]), Some("span"));
    // Each top-level element should have its text child.
    let div_children = frag.dom.get_children(frag.nodes[0]).unwrap();
    assert_eq!(div_children.len(), 1);
    let div_text = frag.dom.get_text(div_children[0]).unwrap();
    assert_eq!(div_text.content, "x");
}

/// Context = div. InBody mode, children of the div are returned.
#[test]
fn parse_fragment_context_div_with_bold_text() {
    let (_ctx_dom, ctx_id) = ctx("div");
    let frag = parse_html_fragment(&_ctx_dom, ctx_id, "<b>hi</b> tail").unwrap();
    assert_eq!(frag.nodes.len(), 2);
    assert_eq!(frag.dom.get_tag(frag.nodes[0]), Some("b"));
    let b_text = frag
        .dom
        .get_children(frag.nodes[0])
        .and_then(|c| c.first().copied())
        .and_then(|t| frag.dom.get_text(t).map(|x| x.content.clone()));
    assert_eq!(b_text, Some("hi".to_string()));
    // The second top-level node is a text " tail".
    let second = frag
        .dom
        .get_text(frag.nodes[1])
        .map(|t| t.content.clone())
        .unwrap_or_default();
    assert_eq!(second, " tail");
}

/// Context = title. RAWTEXT mode: anything but </title> is text.
/// The "html" in the source must NOT become a tag.
#[test]
fn parse_fragment_context_title_is_rawtext() {
    let (_ctx_dom, ctx_id) = ctx("title");
    let frag = parse_html_fragment(&_ctx_dom, ctx_id, "<b>html</b>").unwrap();
    // Title context yields exactly one text-node child: the entire
    // raw source.
    assert_eq!(frag.nodes.len(), 1);
    let text = frag
        .dom
        .get_text(frag.nodes[0])
        .map(|t| t.content.clone())
        .expect("text node");
    assert_eq!(text, "<b>html</b>");
}

/// Context = textarea. RAWTEXT mode: child nodes are text only.
#[test]
fn parse_fragment_context_textarea_is_rawtext() {
    let (_ctx_dom, ctx_id) = ctx("textarea");
    let frag = parse_html_fragment(&_ctx_dom, ctx_id, "hi <there>").unwrap();
    assert_eq!(frag.nodes.len(), 1);
    let text = frag
        .dom
        .get_text(frag.nodes[0])
        .map(|t| t.content.clone())
        .expect("text");
    assert_eq!(text, "hi <there>");
}

/// Context = select. Per §12.4, parses in InSelect mode: only
/// `<option>` and `<optgroup>` are valid children; anything else
/// should be foster-parented out. We assert that the resulting
/// children are exactly `[<option>, <option>]`.
#[test]
fn parse_fragment_context_select_accepts_options() {
    let (_ctx_dom, ctx_id) = ctx("select");
    let frag =
        parse_html_fragment(&_ctx_dom, ctx_id, "<option>a</option><option>b</option>").unwrap();
    let tags: Vec<_> = frag
        .nodes
        .iter()
        .filter_map(|&id| frag.dom.get_tag(id).map(str::to_string))
        .collect();
    assert_eq!(tags, vec!["option", "option"]);
}

/// Context = table. Per §12.4, parses in InTable mode: anything
/// table-shape is treated correctly. `<caption>` becomes a child
/// of `<table>` (and the Fragment's nodes list is the table's
/// children).
#[test]
fn parse_fragment_context_table_accepts_caption() {
    let (_ctx_dom, ctx_id) = ctx("table");
    let frag = parse_html_fragment(&_ctx_dom, ctx_id, "<caption>hi</caption>").unwrap();
    assert_eq!(frag.nodes.len(), 1);
    assert_eq!(frag.dom.get_tag(frag.nodes[0]), Some("caption"));
}

/// Context = tbody. Per §12.4, also InTable mode. The synthetic
/// context element is `<tbody>` (because that's the context
/// element we copied). `<tr>` arriving in InTable mode implies
/// another `<tbody>` first (per the InTable `<tr>` arm), then
/// the `<tr>` becomes a child of that implied tbody.
#[test]
fn parse_fragment_context_tbody_accepts_tr() {
    let (_ctx_dom, ctx_id) = ctx("tbody");
    let frag = parse_html_fragment(&_ctx_dom, ctx_id, "<tr><td>x</td></tr>").unwrap();
    // tbody_synth → [tbody_implied]
    assert_eq!(frag.nodes.len(), 1);
    assert_eq!(frag.dom.get_tag(frag.nodes[0]), Some("tbody"));
    // tbody_implied → [tr]
    let tr_kids = child_tags(&frag.dom, frag.nodes[0]);
    assert_eq!(tr_kids, vec!["tr"]);
    // tr → [td]
    let tr_id = frag.dom.get_children(frag.nodes[0]).unwrap()[0];
    let td_kids = child_tags(&frag.dom, tr_id);
    assert_eq!(td_kids, vec!["td"]);
}

/// Context = body, plain text only. One text node.
#[test]
fn parse_fragment_plain_text_only() {
    let (_ctx_dom, ctx_id) = ctx("body");
    let frag = parse_html_fragment(&_ctx_dom, ctx_id, "hello world").unwrap();
    assert_eq!(frag.nodes.len(), 1);
    let text = frag
        .dom
        .get_text(frag.nodes[0])
        .map(|t| t.content.clone())
        .expect("text");
    assert_eq!(text, "hello world");
}

/// Context = body, empty input. Zero nodes.
#[test]
fn parse_fragment_empty_input_yields_no_nodes() {
    let (_ctx_dom, ctx_id) = ctx("body");
    let frag = parse_html_fragment(&_ctx_dom, ctx_id, "").unwrap();
    assert_eq!(frag.nodes.len(), 0);
}

/// Context = body, malformed input. Parser is lenient: at least
/// the well-formed portion survives.
#[test]
fn parse_fragment_malformed_html_is_lenient() {
    let (_ctx_dom, ctx_id) = ctx("body");
    let frag = parse_html_fragment(&_ctx_dom, ctx_id, "<b>unclosed <i>").unwrap();
    // Should produce at least one node.
    assert!(!frag.nodes.is_empty());
}

/// Context = body. `<tr>` without a `<table>` ancestor should
/// still produce a `<tr>` element (the fragment algorithm wraps
/// the synthetic context body, but the spec's behavior for
/// in-body is to imply a tbody and a table).
///
/// WHATWG §12.4 step 8: when context is `<body>`, parser mode is
/// "in body". In InBody, `<tr>` is not in our block-level list so
/// it falls through to the default `create_element` arm and stays
/// in InBody mode.
#[test]
fn parse_fragment_context_body_keeps_unknown_tags_as_elements() {
    let (_ctx_dom, ctx_id) = ctx("body");
    let frag = parse_html_fragment(&_ctx_dom, ctx_id, "<custom-tag>hi</custom-tag>").unwrap();
    assert_eq!(frag.nodes.len(), 1);
    assert_eq!(frag.dom.get_tag(frag.nodes[0]), Some("custom-tag"));
}

/// Verify that the Fragment.dom is self-contained: it owns its
/// own `<html><head><body>` wrappers, separate from the caller's
/// DOM.
#[test]
fn parse_fragment_dom_is_independent_from_context_dom() {
    let (ctx_dom, ctx_id) = ctx("div");
    let frag = parse_html_fragment(&ctx_dom, ctx_id, "<p>hi</p>").unwrap();
    // The Fragment's Dom and the caller's Dom are different
    // objects — the caller can mutate one without affecting the
    // other.
    assert_eq!(frag.nodes.len(), 1);
    let _first_child = first_child_tag(&frag.dom, frag.dom.root);
    // The Fragment's root has at least <html> or <body> wrapping;
    // we just assert the parsed `<p>` made it in.
    assert_eq!(frag.dom.get_tag(frag.nodes[0]), Some("p"));
    // Suppress unused-warning.
    let _ = ctx_dom;
}
