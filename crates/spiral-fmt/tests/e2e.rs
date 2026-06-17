//! End-to-end tests for `spiral_fmt::parse_html`.
//!
//! These mirror the 6 test cases the pre-existing
//! `spiral-html` crate was running against the now-broken
//! upstream `html5ever` 0.39. With `spiral-fmt` in place,
//! they should all pass. Chunk 3 of the gap-analysis plan
//! rewires `spiral-html` to use this entry point, at which
//! point the original `spiral-html` tests will be replaced
//! by these.

use spiral_dom::Dom;
use spiral_fmt::parse_html;

#[test]
fn parse_simple_div() {
    let dom = parse_html("<div>Hello</div>").expect("parse");
    let root_children = dom.get_children(dom.root).expect("root has children");
    assert!(!root_children.is_empty(), "root should have children");
}

#[test]
fn parse_attributes() {
    let dom = parse_html(r#"<div id="main" class="container">Hi</div>"#).expect("parse");
    let mut found_div = false;
    for (id, _) in dom.descendants(dom.root) {
        if dom.get_tag(id) == Some("div") {
            let attrs = dom.get_attributes(id).expect("div attrs");
            assert!(
                attrs.iter().any(|(k, v)| k == "id" && v == "main"),
                "id attribute missing"
            );
            assert!(
                attrs.iter().any(|(k, v)| k == "class" && v == "container"),
                "class attribute missing"
            );
            found_div = true;
        }
    }
    assert!(found_div, "expected a <div> descendant");
}

#[test]
fn parse_nested_elements() {
    let dom = parse_html("<html><head><title>Test</title></head><body><p>Hi</p></body></html>")
        .expect("parse");
    let tags: Vec<&str> = dom
        .descendants(dom.root)
        .filter_map(|(id, _)| dom.get_tag(id))
        .collect();
    assert!(tags.contains(&"title"), "expected <title>, got {tags:?}");
    assert!(tags.contains(&"p"), "expected <p>, got {tags:?}");
}

#[test]
fn parse_text_merging() {
    let dom = parse_html("<p>Hello World</p>").expect("parse");
    for (id, _) in dom.descendants(dom.root) {
        if dom.get_tag(id) == Some("p") {
            let children = dom.get_children(id).expect("p children");
            let texts: Vec<&str> = children
                .iter()
                .filter_map(|&c| dom.get_text(c).map(|t| t.content.as_str()))
                .collect();
            assert_eq!(
                texts.len(),
                1,
                "expected 1 merged text node, got {}",
                texts.len()
            );
            assert_eq!(texts[0], "Hello World");
            return;
        }
    }
    panic!("no <p> found");
}

#[test]
fn parse_malformed_html_is_lenient() {
    let result = parse_html("<div><span>unclosed<p>reopened</div>");
    assert!(
        result.is_ok(),
        "malformed HTML must be handled leniently: {:?}",
        result.err()
    );
}

#[test]
fn parse_doctype() {
    let dom = parse_html("<!DOCTYPE html><html><head></head><body></body></html>").expect("parse");
    assert!(!dom.get_children(dom.root).unwrap().is_empty());
}

#[test]
fn parse_empty_document() {
    let dom = parse_html("").expect("parse");
    assert_eq!(dom.root, 0);
    // The document has at most one (empty) children vec.
    let _ = dom.get_children(dom.root);
}

#[test]
fn parse_implicit_html_head_body() {
    // Even with no explicit <html>, the DOM should contain
    // auto-inserted <html><head><body>.
    let dom = parse_html("<p>just a paragraph</p>").expect("parse");
    let mut has_html = false;
    let mut has_body = false;
    for (id, _) in dom.descendants(dom.root) {
        match dom.get_tag(id) {
            Some("html") => has_html = true,
            Some("body") => has_body = true,
            _ => {}
        }
    }
    assert!(has_html, "expected auto-inserted <html>");
    assert!(has_body, "expected auto-inserted <body>");
}

#[test]
fn parse_quirks_mode_for_unknown_doctype() {
    let dom = parse_html("<!DOCTYPE weird><html><head></head><body></body></html>").expect("parse");
    let doc = match dom.get_node(dom.root) {
        Some(spiral_dom::Node::Document(d)) => d,
        other => panic!("expected Document at root, got {other:?}"),
    };
    assert!(doc.quirks_mode, "unknown DOCTYPE must trigger quirks mode");
}

#[test]
fn parse_no_quirks_for_html5_doctype() {
    let dom = parse_html("<!DOCTYPE html><html><head></head><body></body></html>").expect("parse");
    let doc = match dom.get_node(dom.root) {
        Some(spiral_dom::Node::Document(d)) => d,
        other => panic!("expected Document at root, got {other:?}"),
    };
    assert!(!doc.quirks_mode, "<!DOCTYPE html> must not trigger quirks");
}

#[test]
fn parse_void_element_does_not_push_to_stack() {
    // A <br> tag should be inserted as a child of <body> and
    // not become a parent of subsequent content.
    let dom = parse_html("<p>before<br>after</p>").expect("parse");
    for (id, _) in dom.descendants(dom.root) {
        if dom.get_tag(id) == Some("p") {
            let children = dom.get_children(id).expect("p children");
            // <p> should have a text child "before" + <br> + text "after"
            assert_eq!(children.len(), 3, "expected 3 children of <p>");
            return;
        }
    }
    panic!("no <p>");
}

#[test]
fn parse_comment_becomes_comment_node() {
    let dom = parse_html("<div>before<!-- hi -->after</div>").expect("parse");
    let mut found_comment = false;
    for (id, _) in dom.descendants(dom.root) {
        if let Some(c) = dom.get_node(id).and_then(|n| match n {
            spiral_dom::Node::Comment(c) => Some(c),
            _ => None,
        }) {
            assert_eq!(c.content, " hi ");
            found_comment = true;
        }
    }
    assert!(found_comment, "expected a Comment node in the DOM");
}

#[test]
fn parse_self_closing_void_attribute() {
    // Self-closing syntax (`<br/>`) is non-conforming HTML
    // for non-foreign void elements. The lenient parser
    // accepts it and treats <br> as a void element.
    let dom = parse_html("<div><br/></div>").expect("parse");
    let mut found_br = false;
    for (id, _) in dom.descendants(dom.root) {
        if dom.get_tag(id) == Some("br") {
            found_br = true;
        }
    }
    assert!(found_br, "expected a <br> element");
    // Use the Dom in the assertion to silence unused warnings.
    let _ = Dom::new();
}

// ----------------------------------------------------------------
// Rawtext / script-data mode (M4.4.1 Item 2)
//
// These exercise the full tree-builder → tokeniser path for
// the four cases the audit flagged: <script>, <style>,
// <textarea>, <title>. A `<` inside the body of these elements
// must be delivered as text, not parsed as a tag-open.
// ----------------------------------------------------------------

fn find_text_under(dom: &spiral_dom::Dom, tag: &str) -> String {
    for (id, _) in dom.descendants(dom.root) {
        if dom.get_tag(id) == Some(tag) {
            let children = dom.get_children(id).expect("children");
            let mut out = String::new();
            for c in children {
                if let Some(t) = dom.get_text(c) {
                    out.push_str(&t.content);
                }
            }
            return out;
        }
        // For <title>, which may be on the stack and a
        // descendant scan will reach it; for <textarea> /
        // <style> / <script>, same.
    }
    String::new()
}

#[test]
fn parse_script_inner_lt_is_text() {
    // The headline case from the audit: a `<` inside <script>
    // must not be tokenised as a tag-open.
    let dom = parse_html("<script>if (a < b) { return; }</script>").expect("parse");
    let body = find_text_under(&dom, "script");
    assert_eq!(body, "if (a < b) { return; }");
}

#[test]
fn parse_style_inner_lt_is_text() {
    let dom = parse_html("<style>div > a { color: red; }</style>").expect("parse");
    let body = find_text_under(&dom, "style");
    assert_eq!(body, "div > a { color: red; }");
}

#[test]
fn parse_textarea_inner_lt_is_text() {
    // Per HTML5, `<textarea>` is rawtext: no character
    // reference decoding happens inside it. The literal
    // `&amp;` is therefore preserved verbatim, and `<` is
    // preserved verbatim. This is the behaviour the audit
    // flagged as missing.
    let dom = parse_html("<textarea>5 < 10 &amp; 10 > 3</textarea>").expect("parse");
    let body = find_text_under(&dom, "textarea");
    assert_eq!(body, "5 < 10 &amp; 10 > 3");
}

#[test]
fn parse_title_inner_lt_is_text() {
    let dom = parse_html("<title>if a < b then go</title>").expect("parse");
    let body = find_text_under(&dom, "title");
    assert_eq!(body, "if a < b then go");
}

#[test]
fn parse_script_with_closing_tag_terminates() {
    // The matching `</script>` must terminate the body, even
    // when the end tag is uppercase.
    let dom = parse_html("<script>x = 1;</SCRIPT>tail").expect("parse");
    let body = find_text_under(&dom, "script");
    assert_eq!(body, "x = 1;");
    // The `tail` text should land outside the <script>
    // element.
    assert!(
        dom.descendants(dom.root).any(|(id, _)| {
            dom.get_tag(id).is_none()
                && dom
                    .get_text(id)
                    .map(|t| t.content == "tail")
                    .unwrap_or(false)
        }),
        "expected a text node 'tail' after </script>"
    );
}

// ----------------------------------------------------------------
// Numeric character references (M4.4.1 Item 3)
//
// `&#NN;` (decimal) and `&#xHH;` / `&#XHH;` (hex) per HTML5
// §13.2.5.72-78. These cover the spec-mandated replacement
// table for null, surrogates, out-of-range, and the 0x80..=0x9F
// Windows-1252 fixup.
// ----------------------------------------------------------------

fn find_text_in_body(dom: &spiral_dom::Dom) -> String {
    // Concatenate all text nodes anywhere in the document
    // (the M4.4.1 tree builder inserts text under whatever
    // element is current, not directly under <body>).
    let mut out = String::new();
    for (id, _) in dom.descendants(dom.root) {
        if let Some(t) = dom.get_text(id) {
            out.push_str(&t.content);
        }
    }
    out
}

#[test]
fn parse_decimal_numeric_ref_in_body() {
    let dom = parse_html("<p>x&#65;y</p>").expect("parse");
    assert_eq!(find_text_in_body(&dom), "xAy");
}

#[test]
fn parse_hex_numeric_ref_in_body() {
    let dom = parse_html("<p>x&#x41;y</p>").expect("parse");
    assert_eq!(find_text_in_body(&dom), "xAy");
}

#[test]
fn parse_numeric_ref_in_attribute_value() {
    // Numeric references decode inside attribute values.
    let dom = parse_html(r#"<a href="?q=&#65;">x</a>"#).expect("parse");
    for (id, _) in dom.descendants(dom.root) {
        if dom.get_tag(id) == Some("a") {
            let attrs = dom.get_attributes(id).expect("a has attrs");
            assert_eq!(attrs[0].1, "?q=A");
        }
    }
}

#[test]
fn parse_numeric_ref_unicode_above_bmp() {
    // `&#x1F600;` is a non-BMP code point (😀, GRINNING
    // FACE). The replacement must produce a 4-byte UTF-8
    // sequence, not two surrogate halves or FFFD.
    let dom = parse_html("<p>&#x1F600;</p>").expect("parse");
    assert_eq!(find_text_in_body(&dom), "😀");
}

#[test]
fn parse_numeric_ref_euro_sign() {
    // `&#x80;` → EURO SIGN (U+20AC) per the 0x80..=0x9F
    // Windows-1252 fixup table.
    let dom = parse_html("<p>&#x80;</p>").expect("parse");
    assert_eq!(find_text_in_body(&dom), "\u{20AC}");
}

#[test]
fn parse_numeric_ref_null_replacement() {
    // `&#0;` is replaced with U+FFFD, not emitted as a NUL.
    let dom = parse_html("<p>&#0;</p>").expect("parse");
    assert_eq!(find_text_in_body(&dom), "\u{FFFD}");
}

#[test]
fn parse_named_and_numeric_mix() {
    // Named and numeric references can appear in the same
    // text run.
    let dom = parse_html("<p>&amp;&#65;&#x42;</p>").expect("parse");
    assert_eq!(find_text_in_body(&dom), "&AB");
}

// ------------------------------------------------------------
// CSS end-to-end tests
// ------------------------------------------------------------
//
// These exercise `spiral_fmt::parse_css`, the M4.4.1
// minimum-viable CSS parser. The tests cover the surface
// the rest of the M4 sprint and the M5 cascade depend on:
// qualified rules with declarations, at-rules (block and
// terminator forms), selector specificity, attribute
// selectors with the case flag, pseudo-classes, and the
// `!important` marker.

use spiral_fmt::{
    parse_css, AttributeCase, AttributeMatcher, Combinator, Rule, TypeSelector, Value,
};

#[test]
fn css_parse_qualified_rule_simple() {
    let sheet = parse_css("p { color: red; }").expect("parse");
    let rules: Vec<_> = sheet.qualified_rules().collect();
    assert_eq!(rules.len(), 1);
    let q = rules[0];
    assert_eq!(q.declarations.len(), 1);
    assert_eq!(q.declarations[0].name, "color");
    // The parser resolves named colours to `Value::Color`.
    assert!(matches!(q.declarations[0].value, Value::Color(_)));
    assert!(!q.declarations[0].important);
}

#[test]
fn css_parse_qualified_rule_multiple_declarations() {
    let sheet =
        parse_css("div { display: block; margin: 0 auto; padding: 10px 20px; }").expect("parse");
    let q = sheet.qualified_rules().next().expect("rule");
    assert_eq!(q.declarations.len(), 3);
    assert_eq!(q.declarations[0].name, "display");
    assert_eq!(q.declarations[1].name, "margin");
    assert_eq!(q.declarations[2].name, "padding");
    // `padding: 10px 20px` parses as a 2-element list.
    match &q.declarations[2].value {
        Value::List(items) => {
            assert_eq!(items.len(), 2);
            assert!(matches!(items[0], Value::Length(10.0)));
            assert!(matches!(items[1], Value::Length(20.0)));
        }
        other => panic!("expected padding list, got {:?}", other),
    }
}

#[test]
fn css_parse_selector_list_alternatives() {
    let sheet = parse_css("h1, h2, h3 { font-weight: bold; }").expect("parse");
    let q = sheet.qualified_rules().next().expect("rule");
    assert_eq!(q.selector.alternatives.len(), 3);
    let types: Vec<&str> = q
        .selector
        .alternatives
        .iter()
        .map(|alt| {
            let step = &alt.steps[0];
            match &step.compound.type_selector {
                Some(TypeSelector::Element(name)) => name.as_str(),
                other => panic!("expected element type, got {:?}", other),
            }
        })
        .collect();
    assert_eq!(types, vec!["h1", "h2", "h3"]);
}

#[test]
fn css_parse_combinators_descendant_child_sibling() {
    // Descendant, child, and adjacent-sibling in one
    // selector.
    let sheet = parse_css("article p { color: black; }").expect("parse");
    let q = sheet.qualified_rules().next().expect("rule");
    let alt = &q.selector.alternatives[0];
    assert_eq!(alt.steps.len(), 2);
    assert!(matches!(
        alt.steps[0].combinator,
        Some(Combinator::Descendant)
    ));

    let sheet2 = parse_css("ul > li { list-style: none; }").expect("parse");
    let q2 = sheet2.qualified_rules().next().expect("rule");
    let alt2 = &q2.selector.alternatives[0];
    assert!(matches!(alt2.steps[0].combinator, Some(Combinator::Child)));

    let sheet3 = parse_css("h1 + p { margin-top: 0; }").expect("parse");
    let q3 = sheet3.qualified_rules().next().expect("rule");
    let alt3 = &q3.selector.alternatives[0];
    assert!(matches!(
        alt3.steps[0].combinator,
        Some(Combinator::NextSibling)
    ));
}

#[test]
fn css_parse_specificity_class_beats_element() {
    let sheet = parse_css("p { color: red; } .note { color: blue; }").expect("parse");
    let mut element_spec = None;
    let mut class_spec = None;
    for rule in sheet.qualified_rules() {
        let spec = rule.selector.specificity();
        let alt = &rule.selector.alternatives[0];
        let step = &alt.steps[0];
        let is_class = !step.compound.classes.is_empty();
        if is_class {
            class_spec = Some(spec);
        } else {
            element_spec = Some(spec);
        }
    }
    let element_spec = element_spec.expect("element rule");
    let class_spec = class_spec.expect("class rule");
    assert!(
        class_spec > element_spec,
        "class specificity {:?} should beat element {:?}",
        class_spec,
        element_spec
    );
}

#[test]
fn css_parse_specificity_id_beats_class() {
    let sheet = parse_css(".foo { color: red; } #bar { color: blue; }").expect("parse");
    let mut class_spec = None;
    let mut id_spec = None;
    for rule in sheet.qualified_rules() {
        let spec = rule.selector.specificity();
        let step = &rule.selector.alternatives[0].steps[0];
        if !step.compound.ids.is_empty() {
            id_spec = Some(spec);
        } else if !step.compound.classes.is_empty() {
            class_spec = Some(spec);
        }
    }
    let class_spec = class_spec.expect("class rule");
    let id_spec = id_spec.expect("id rule");
    assert!(
        id_spec > class_spec,
        "id specificity {:?} should beat class {:?}",
        id_spec,
        class_spec
    );
}

#[test]
fn css_parse_attribute_selector_present() {
    let sheet = parse_css("[disabled] { cursor: not-allowed; }").expect("parse");
    let q = sheet.qualified_rules().next().expect("rule");
    let attrs = &q.selector.alternatives[0].steps[0].compound.attributes;
    assert_eq!(attrs.len(), 1);
    assert_eq!(attrs[0].name, "disabled");
    assert!(matches!(attrs[0].matcher, AttributeMatcher::Present));
    assert_eq!(attrs[0].case, AttributeCase::Sensitive);
}

#[test]
fn css_parse_attribute_selector_matchers() {
    let sheet = parse_css("[a=x] { } [a~=x] { } [a|=x] { } [a^=x] { } [a$=x] { } [a*=x] { }")
        .expect("parse");
    let expected = [
        AttributeMatcher::Exact("x".to_string()),
        AttributeMatcher::Includes("x".to_string()),
        AttributeMatcher::DashMatch("x".to_string()),
        AttributeMatcher::Prefix("x".to_string()),
        AttributeMatcher::Suffix("x".to_string()),
        AttributeMatcher::Substring("x".to_string()),
    ];
    let actual: Vec<&AttributeMatcher> = sheet
        .qualified_rules()
        .map(|q| &q.selector.alternatives[0].steps[0].compound.attributes[0].matcher)
        .collect();
    assert_eq!(actual.len(), expected.len());
    for (i, want) in expected.iter().enumerate() {
        assert_eq!(*actual[i], *want, "matcher {} mismatch", i);
    }
}

#[test]
fn css_parse_attribute_selector_case_flag() {
    // The `i` flag marks the selector as ASCII case
    // insensitive.
    let sheet = parse_css("[type=text i] { color: red; }").expect("parse");
    let q = sheet.qualified_rules().next().expect("rule");
    let attr = &q.selector.alternatives[0].steps[0].compound.attributes[0];
    assert_eq!(attr.name, "type");
    assert!(matches!(attr.matcher, AttributeMatcher::Exact(ref v) if v == "text"));
    assert_eq!(attr.case, AttributeCase::Insensitive);
}

#[test]
fn css_parse_pseudo_class() {
    let sheet = parse_css("a:hover { text-decoration: underline; }").expect("parse");
    let q = sheet.qualified_rules().next().expect("rule");
    let pseudo = &q.selector.alternatives[0].steps[0].compound.pseudo_classes;
    assert_eq!(pseudo.len(), 1);
    assert_eq!(pseudo[0].0, "hover");
    assert!(pseudo[0].1.is_none());
}

#[test]
fn css_parse_important_marker() {
    let sheet = parse_css("p { color: red !important; }").expect("parse");
    let q = sheet.qualified_rules().next().expect("rule");
    assert_eq!(q.declarations.len(), 1);
    assert!(q.declarations[0].important);
    assert!(matches!(q.declarations[0].value, Value::Color(_)));
}

#[test]
fn css_parse_at_rule_block_form() {
    let sheet = parse_css("@media (min-width: 600px) { p { color: blue; } }").expect("parse");
    let ats: Vec<_> = sheet.at_rules().collect();
    assert_eq!(ats.len(), 1);
    let at = ats[0];
    assert_eq!(at.name, "media");
    let block = at.block.as_ref().expect("@media should have a block");
    assert_eq!(block.rules.len(), 1);
    assert!(matches!(block.rules[0], Rule::Qualified(_)));
}

#[test]
fn css_parse_at_rule_terminator_form() {
    let sheet = parse_css("@import url(\"foo.css\");").expect("parse");
    let ats: Vec<_> = sheet.at_rules().collect();
    assert_eq!(ats.len(), 1);
    let at = ats[0];
    assert_eq!(at.name, "import");
    assert!(
        at.block.is_none(),
        "@import uses the ';' terminator form, not a block"
    );
}

#[test]
fn css_parse_value_colour_lengths_percentages() {
    // A single declaration exercising the numeric value
    // surface: hex colour, length, percentage.
    let sheet = parse_css("p { color: #ff0000; width: 10px; height: 50%; }").expect("parse");
    let q = sheet.qualified_rules().next().expect("rule");
    let decls = &q.declarations;
    assert!(matches!(decls[0].value, Value::Color(_)));
    assert!(matches!(decls[1].value, Value::Length(10.0)));
    assert!(matches!(decls[2].value, Value::Percentage(50.0)));
}

#[test]
fn parse_siblings_nesting() {
    let dom = parse_html("<p>first</p><a>second</a>").expect("parse");
    let mut p_has_children = false;
    let mut root_has_a = false;
    for (id, _) in dom.descendants(dom.root) {
        if dom.get_tag(id) == Some("p") {
            let children = dom.get_children(id).expect("p children");
            for &c in &children {
                if dom.get_tag(c) == Some("a") {
                    panic!("<a> should not be a child of <p>");
                }
            }
            p_has_children = !children.is_empty();
        }
        if dom.get_tag(id) == Some("a") {
            let parent = dom.get_parent(id).expect("a parent");
            if dom.get_tag(parent) == Some("body") {
                root_has_a = true;
            }
        }
    }
    assert!(p_has_children);
    assert!(root_has_a);
}

#[test]
fn parse_aaa_misnested_formatting_tags() {
    let dom = parse_html("<b>bold <i>bold-italic</b> italic</i>").expect("parse");
    let body_id = dom
        .descendants(dom.root)
        .find(|&(id, _)| dom.get_tag(id) == Some("body"))
        .map(|(id, _)| id)
        .expect("body");

    let body_children = dom.get_children(body_id).expect("body children");
    assert_eq!(
        body_children.len(),
        2,
        "body should have exactly 2 children: <b> and <i>"
    );

    let first_child = body_children[0];
    assert_eq!(dom.get_tag(first_child), Some("b"));

    let second_child = body_children[1];
    assert_eq!(dom.get_tag(second_child), Some("i"));

    let b_children = dom.get_children(first_child).expect("b children");
    assert_eq!(b_children.len(), 2);
    assert!(dom.get_text(b_children[0]).map(|t| t.content.as_str()) == Some("bold "));
    assert_eq!(dom.get_tag(b_children[1]), Some("i"));

    let i1_children = dom.get_children(b_children[1]).expect("i1 children");
    assert_eq!(i1_children.len(), 1);
    assert!(dom.get_text(i1_children[0]).map(|t| t.content.as_str()) == Some("bold-italic"));

    let i2_children = dom.get_children(second_child).expect("i2 children");
    assert_eq!(i2_children.len(), 1);
    assert!(dom.get_text(i2_children[0]).map(|t| t.content.as_str()) == Some(" italic"));
}

#[test]
fn parse_afe_noahs_ark_clause() {
    let dom = parse_html("<b><b><b><b>x</b></b></b></b>").expect("parse");
    assert!(!dom.get_children(dom.root).unwrap().is_empty());
}

#[test]
fn parse_aaa_with_furthest_block() {
    let dom = parse_html("<b>bold <p>paragraph</b> tail</p>").expect("parse");
    let body_id = dom
        .descendants(dom.root)
        .find(|&(id, _)| dom.get_tag(id) == Some("body"))
        .map(|(id, _)| id)
        .expect("body");

    let body_children = dom.get_children(body_id).expect("body children");
    assert_eq!(
        body_children.len(),
        2,
        "body should have exactly 2 children: <b> and <p>"
    );

    let first_child = body_children[0];
    assert_eq!(dom.get_tag(first_child), Some("b"));

    let second_child = body_children[1];
    assert_eq!(dom.get_tag(second_child), Some("p"));

    let b_children = dom.get_children(first_child).expect("b children");
    assert_eq!(b_children.len(), 1);
    assert!(dom.get_text(b_children[0]).map(|t| t.content.as_str()) == Some("bold "));

    let p_children = dom.get_children(second_child).expect("p children");
    assert_eq!(p_children.len(), 2);
    assert_eq!(dom.get_tag(p_children[0]), Some("b"));
    assert!(dom.get_text(p_children[1]).map(|t| t.content.as_str()) == Some(" tail"));

    let inner_b_children = dom.get_children(p_children[0]).expect("inner b children");
    assert_eq!(inner_b_children.len(), 1);
    assert!(
        dom.get_text(inner_b_children[0])
            .map(|t| t.content.as_str())
            == Some("paragraph")
    );
}

// =====================================================================
// Foster parenting (Packet 2.8.3 — WHATWG HTML §12.2.6.1)
// =====================================================================
//
// Foster parenting is what the parser does when a non-table tag ends
// up between table children: it lifts the orphan OUT of the table and
// reinserts it just before the table. This is a one-time cost per
// misnesting that real-world pages pay silently all the time.

/// WHATWG §12.2.6.1, example 1:
/// `<table><b>foo</b><tr><td>bar</td></tr></table>`
/// → `<b>` should end up as a SIBLING of `<table>`, not inside it.
/// `<tr>`/`<td>` are still inside the `<table>`.
#[test]
fn parse_foster_parent_inline_before_table_row() {
    let dom = parse_html("<table><b>foo</b><tr><td>bar</td></tr></table>").expect("parse");

    let body_id = dom
        .descendants(dom.root)
        .find(|&(id, _)| dom.get_tag(id) == Some("body"))
        .map(|(id, _)| id)
        .expect("body");

    let body_children = dom.get_children(body_id).expect("body children");
    // body has exactly one child: <b> (fostered) followed by the
    // <table> sibling. We accept any order in which <b> appears
    // before <table>.
    let mut b_before_table = false;
    let mut saw_b = false;
    for &child in &body_children {
        if dom.get_tag(child) == Some("b") {
            saw_b = true;
            // <b>'s text "foo" must be its own child (i.e. <b> is a
            // real element, not a wrapper around the table).
            let b_children = dom.get_children(child).expect("b children");
            assert_eq!(b_children.len(), 1);
            assert!(dom.get_text(b_children[0]).map(|t| t.content.as_str()) == Some("foo"));
            // Crucially, <b> must NOT contain the <table>.
            for c in &b_children {
                assert_ne!(dom.get_tag(*c), Some("table"));
            }
        } else if dom.get_tag(child) == Some("table") {
            // The <table> must be empty of inline content; only the
            // <tr> child should be inside.
            if saw_b {
                b_before_table = true;
            }
            let table_children = dom.get_children(child).expect("table children");
            for tc in &table_children {
                if dom.get_tag(*tc) != Some("tbody")
                    && dom.get_tag(*tc) != Some("thead")
                    && dom.get_tag(*tc) != Some("tfoot")
                {
                    assert_eq!(
                        dom.get_tag(*tc),
                        Some("tr"),
                        "non-tr child inside <table>: {:?}",
                        dom.get_tag(*tc)
                    );
                }
            }
            // The <td> should be reachable inside the table subtree.
            let has_td = dom
                .descendants(child)
                .any(|(id, _)| dom.get_tag(id) == Some("td"));
            assert!(has_td, "<td> should still be inside <table>");
        }
    }
    assert!(saw_b, "<b> should be a sibling of <table>");
    assert!(b_before_table, "<b> should appear before <table> in body");
}

/// WHATWG §12.2.6.1, the text-foster case:
/// `<table>foo<tr><td>bar</td></tr></table>`
/// → the text "foo" is foster-parented BEFORE the table.
#[test]
fn parse_foster_parent_text_before_table() {
    let dom = parse_html("<table>foo<tr><td>bar</td></tr></table>").expect("parse");
    let body_id = dom
        .descendants(dom.root)
        .find(|&(id, _)| dom.get_tag(id) == Some("body"))
        .map(|(id, _)| id)
        .expect("body");

    let body_children = dom.get_children(body_id).expect("body children");
    let mut table_id = None;
    for &c in &body_children {
        if dom.get_tag(c) == Some("table") {
            table_id = Some(c);
        }
    }
    let table_id = table_id.expect("table in body");

    // The text "foo" should be a text-node sibling of <table> that
    // appears BEFORE <table> in the body.
    let mut saw_foo = false;
    for &c in &body_children {
        if c == table_id {
            // Once we see the table, the fostered text must already
            // have appeared.
            assert!(saw_foo, "fostered text must appear before <table>");
            break;
        }
        if let Some(t) = dom.get_text(c) {
            if t.content == "foo" {
                saw_foo = true;
            }
        }
    }

    // <table> itself must not contain the text "foo".
    for (id, _) in dom.descendants(table_id) {
        if let Some(t) = dom.get_text(id) {
            assert_ne!(t.content, "foo");
        }
    }
}

/// WHATWG §12.2.6.1, the <select> case:
/// `<select><option>x<b>y</b></option></select>`
/// → `<b>` stays inside `<option>` (no foster parenting in select).
/// The reverse: `<select><b>foo</b><option>bar</option></select>`
/// → `<b>` is also foster-parented OUT of select in the spec (since
/// `<select>` is a "special" parent). We assert the conservative
/// reading: `<b>` is a child of the body / ancestor of `<select>`,
/// not inside `<select>`.
#[test]
fn parse_foster_parent_select_kicks_inline() {
    let dom = parse_html("<select><b>foo</b><option>bar</option></select>").expect("parse");
    let body_id = dom
        .descendants(dom.root)
        .find(|&(id, _)| dom.get_tag(id) == Some("body"))
        .map(|(id, _)| id)
        .expect("body");

    // <b> must not be a descendant of <select>.
    let select_id = dom
        .descendants(body_id)
        .find(|&(id, _)| dom.get_tag(id) == Some("select"))
        .map(|(id, _)| id)
        .expect("select");
    let b_inside_select = dom
        .descendants(select_id)
        .any(|(id, _)| dom.get_tag(id) == Some("b"));
    assert!(
        !b_inside_select,
        "<b> should be foster-parented out of <select>"
    );

    // <b> must still exist somewhere in the document.
    let b_anywhere = dom
        .descendants(dom.root)
        .any(|(id, _)| dom.get_tag(id) == Some("b"));
    assert!(b_anywhere, "<b> should still exist in the document");
}
