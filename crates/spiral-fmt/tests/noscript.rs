//! End-to-end tests for the `<noscript>` element.
//!
//! WHATWG HTML §4.6.7 + §13 tree-builder handling.
//!
//! In a v0.1 parser with the scripting flag **always on**, the
//! spec says: a `<noscript>` start tag in `<head>` is treated as
//! an ordinary metadata element (its contents parse as regular
//! HTML), but a `<noscript>` start tag anywhere else switches the
//! tokeniser to RAWTEXT mode (so its children are taken as
//! opaque text, not parsed as tags).
//!
//! In a v0.1 parser with the scripting flag **always off**,
//! `<noscript>` is a regular element everywhere — its children
//! are parsed normally.
//!
//! Spiral's v0.1 ships with the scripting flag always on. The
//! packet 2.1.3 wiring is therefore:
//!
//! - `<head><noscript>...</noscript></head>`: contents parse as
//!   regular HTML (no rawtext switch) — this is the spec-mandated
//!   "in head" behaviour.
//! - `<body><noscript>...</noscript></body>`: contents are taken
//!   as raw text (rawtext switch on) — this is the spec-mandated
//!   "in body" behaviour.
//! - The end tag `</noscript>` always pops the current node off
//!   the open-elements stack.
//! - `noscript` is registered as a normal element in the
//!   metadata element set so the `<head>` insertion-mode
//!   accept-list at `spiral_fmt::html::tree::TreeBuilder::handle_start_tag`
//!   (InHead arm, line 392) lets it through.
//!
//! These tests are the wiring-of-truth for packet 2.1.3. They
//! exercise the public entry point `spiral_fmt::parse_html` and
//! inspect the resulting DOM.

use spiral_dom::{Dom, NodeId};
use spiral_fmt::parse_html;

fn parse(source: &str) -> Dom {
    parse_html(source).expect("parse should succeed")
}

fn tags_under(dom: &Dom, root: NodeId) -> Vec<String> {
    let mut out = Vec::new();
    for (id, _) in dom.descendants(root) {
        if let Some(t) = dom.get_tag(id) {
            out.push(t.to_string());
        }
    }
    out
}

#[test]
fn noscript_in_head_parses_children_as_html() {
    let dom = parse("<head><noscript><link rel=\"stylesheet\" href=\"x.css\"></noscript></head>");
    let tags = tags_under(&dom, dom.root);
    assert!(
        tags.iter().any(|t| t == "noscript"),
        "expected <noscript> in tree, got {tags:?}"
    );
    assert!(
        tags.iter().any(|t| t == "link"),
        "expected <link> as a parsed child of <noscript> in <head>, got {tags:?}"
    );
}

#[test]
fn noscript_in_head_with_paragraph() {
    let dom = parse("<head><noscript><p>fallback</p></noscript></head>");
    let tags = tags_under(&dom, dom.root);
    assert!(
        tags.iter().any(|t| t == "p"),
        "expected <p> as a parsed child of <noscript> in <head>, got {tags:?}"
    );
}

#[test]
fn noscript_in_body_treats_children_as_text() {
    let dom = parse("<body><noscript>raw <em>not a tag</em></noscript></body>");
    let mut found_noscript = false;
    for (id, _) in dom.descendants(dom.root) {
        if dom.get_tag(id) == Some("noscript") {
            found_noscript = true;
            let children: Vec<NodeId> =
                dom.get_children(id).map(|c| c.to_vec()).unwrap_or_default();
            let mut has_em = false;
            let mut has_raw_text = false;
            for c in children {
                if dom.get_tag(c) == Some("em") {
                    has_em = true;
                }
                if let Some(text) = dom.get_text(c) {
                    if text.content.contains("not a tag") {
                        has_raw_text = true;
                    }
                }
            }
            assert!(
                has_raw_text,
                "expected raw text inside <body><noscript>, no <em> child should be created"
            );
            assert!(
                !has_em,
                "expected <em> NOT to be a parsed child of <body><noscript>"
            );
        }
    }
    assert!(found_noscript, "expected a <noscript> in the tree");
}

#[test]
fn noscript_end_tag_pops_the_open_element() {
    let dom = parse("<head><noscript><p>a</p></noscript><meta name=\"b\"></head>");
    let tags = tags_under(&dom, dom.root);
    let noscript_idx = tags.iter().position(|t| t == "noscript");
    let meta_idx = tags.iter().position(|t| t == "meta");
    let p_idx = tags.iter().position(|t| t == "p");
    assert!(
        noscript_idx.is_some() && meta_idx.is_some() && p_idx.is_some(),
        "expected <noscript>, <p>, and <meta> in tree, got {tags:?}"
    );
    let n = noscript_idx.unwrap();
    let m = meta_idx.unwrap();
    let p = p_idx.unwrap();
    assert!(
        n < p && p < m,
        "expected order <noscript>(idx={n}) < <p>(idx={p}) < <meta>(idx={m}), got {tags:?}"
    );
}

#[test]
fn noscript_is_recognised_in_head() {
    let dom = parse("<head><noscript><span>x</span></noscript></head>");
    let tags = tags_under(&dom, dom.root);
    assert!(
        tags.iter().any(|t| t == "noscript"),
        "expected <noscript> as a parsed element in <head>, got {tags:?}"
    );
    assert!(
        tags.iter().any(|t| t == "span"),
        "expected <span> as a parsed child of <noscript> in <head>, got {tags:?}"
    );
}

#[test]
fn noscript_with_text_only_in_head() {
    let dom = parse("<head><noscript>just text</noscript></head>");
    let mut found = false;
    for (id, _) in dom.descendants(dom.root) {
        if dom.get_tag(id) == Some("noscript") {
            found = true;
            let children: Vec<NodeId> =
                dom.get_children(id).map(|c| c.to_vec()).unwrap_or_default();
            let mut has_text = false;
            for c in children {
                if let Some(t) = dom.get_text(c) {
                    if t.content.contains("just text") {
                        has_text = true;
                    }
                }
            }
            assert!(
                has_text,
                "expected the text 'just text' inside <noscript> in <head>"
            );
        }
    }
    assert!(found, "expected a <noscript> in the tree");
}

#[test]
fn noscript_in_body_attributes_are_kept() {
    let dom = parse("<body><noscript id=\"ns1\" class=\"fb\">fallback</noscript></body>");
    let mut found = false;
    for (id, _) in dom.descendants(dom.root) {
        if dom.get_tag(id) == Some("noscript") {
            found = true;
            let attrs = dom.get_attributes(id).expect("noscript attrs");
            assert!(
                attrs.iter().any(|(k, v)| k == "id" && v == "ns1"),
                "expected id=\"ns1\" on <noscript>, got {attrs:?}"
            );
            assert!(
                attrs.iter().any(|(k, v)| k == "class" && v == "fb"),
                "expected class=\"fb\" on <noscript>, got {attrs:?}"
            );
        }
    }
    assert!(found, "expected a <noscript> in the tree");
}
