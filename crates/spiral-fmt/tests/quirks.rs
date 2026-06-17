//! End-to-end tests for the quirks-mode classifier.
//!
//! WHATWG HTML §13.2.2.5 ("Parsing the DOCTYPE") defines the
//! rules that drive whether the document is in quirks, limited
//! quirks, or no quirks mode. The classifier lives in
//! `spiral_fmt::html::tokeniser::classify_doctype_quirks` and is
//! wired into the tree builder at
//! `spiral_fmt::html::tree::TreeBuilder::handle_doctype`. The
//! result is exposed on the parsed DOM via
//! `spiral_dom::Dom::quirks_mode`.
//!
//! These tests exercise the public surface end-to-end:
//! `parse_html(...).quirks_mode()`. A bare `<!DOCTYPE html>`
//! must not trigger quirks. Anything else must.

use spiral_dom::Dom;
use spiral_fmt::parse_html;

fn parse(source: &str) -> Dom {
    parse_html(source).expect("parse should succeed")
}

#[test]
fn parse_doctype_html5_no_quirks() {
    let dom = parse("<!DOCTYPE html><p>x");
    assert!(
        !dom.quirks_mode(),
        "<!DOCTYPE html> must produce a no-quirks document"
    );
}

#[test]
fn parse_doctype_unknown_triggers_quirks() {
    let dom = parse("<!DOCTYPE weird><p>x");
    assert!(
        dom.quirks_mode(),
        "an unknown DOCTYPE name must trigger quirks mode"
    );
}

#[test]
fn parse_doctype_missing_triggers_quirks() {
    let dom = parse("<!DOCTYPE><p>x");
    assert!(
        dom.quirks_mode(),
        "a missing DOCTYPE name must trigger quirks mode per §13.2.2.5 step 7"
    );
}

#[test]
fn parse_no_doctype_triggers_quirks() {
    let dom = parse("<html><head></head><body></body></html>");
    assert!(
        dom.quirks_mode(),
        "the absence of a DOCTYPE must put the document into quirks mode per §12.1"
    );
}

#[test]
fn parse_doctype_html4_strict_no_quirks() {
    // No-quirks PUBLIC triple per §13.2.2.5: HTML 4.01 Strict.
    let dom = parse(
        r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd"><p>x"#,
    );
    assert!(
        !dom.quirks_mode(),
        "HTML 4.01 Strict DOCTYPE must not trigger quirks"
    );
}

#[test]
fn parse_doctype_html4_transitional_limited_quirks() {
    // Limited-quirks PUBLIC triple per §13.2.2.5: HTML 4.01 Transitional.
    // Limited quirks does not enable CSS quirks, so `quirks_mode()` stays
    // false. (We distinguish "limited quirks" vs "no quirks" only at the
    // token level for now; `Dom::quirks_mode` is the single boolean CSS
    // hook the rendering layer reads.)
    let dom = parse(
        r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd"><p>x"#,
    );
    assert!(
        !dom.quirks_mode(),
        "HTML 4.01 Transitional must not trigger full quirks mode"
    );
}

#[test]
fn parse_doctype_html4_frameset_triggers_quirks() {
    // Full-quirks PUBLIC triple per §13.2.2.5: HTML 4.01 Frameset.
    let dom = parse(
        r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Frameset//EN" "http://www.w3.org/TR/html4/frameset.dtd"><p>x"#,
    );
    assert!(
        dom.quirks_mode(),
        "HTML 4.01 Frameset DOCTYPE must trigger quirks mode"
    );
}

#[test]
fn parse_doctype_case_insensitive_name() {
    // Name comparison is ASCII-case-insensitive per §13.2.2.5.
    let dom = parse("<!DOCTYPE HTML><p>x");
    assert!(
        !dom.quirks_mode(),
        "<!DOCTYPE HTML> (uppercase) must be classified identically to <!DOCTYPE html>"
    );
}

#[test]
fn parse_doctype_public_id_with_unknown_name_triggers_quirks() {
    // Unknown name with a PUBLIC id still triggers quirks per step 6.
    let dom = parse(r#"<!DOCTYPE foo PUBLIC "bar" "baz"><p>x"#);
    assert!(
        dom.quirks_mode(),
        "unknown DOCTYPE name with PUBLIC id must trigger quirks"
    );
}

#[test]
fn parse_quirks_mode_default_is_true() {
    // Document::quirks_mode defaults to true; a fresh Dom without a
    // DOCTYPE-driven setter must surface quirks. This guards against
    // accidental "default false" regressions in spiral-dom.
    let dom = parse("<p>x");
    assert!(
        dom.quirks_mode(),
        "default quirks_mode on a fresh Dom must be true (quirks by default)"
    );
}
