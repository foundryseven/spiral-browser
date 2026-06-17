//! WHATWG HTML §12.4 HTML fragment parsing algorithm.
//!
//! The fragment algorithm is what `Element.innerHTML = "..."`,
//! `<template>` content document-fragment construction, and the
//! Vortex `Element.innerHTML` JS binding all depend on.
//!
//! ## Differences from the document algorithm
//!
//! The document parser (see [`super::parse`]) is fed a stream of
//! tokens and progresses through `Initial → BeforeHtml → BeforeHead
//! → InHead → AfterHead → InBody → ...` as it sees the relevant
//! tags. The fragment parser instead:
//!
//! 1. Builds the same synthetic `<html><head><body>` wrappers,
//! 2. **Pushes a synthetic copy of the context element** onto the
//!    stack,
//! 3. **Sets the insertion mode** based on the context element's
//!    tag name (the spec's context-to-mode table — see below),
//! 4. Tokenises the input and feeds it through the regular
//!    insertion-mode machine.
//!
//! After EOF, the fragment's top-level nodes are extracted from
//! the DOM (the children of the synthetic context element) and
//! returned to the caller.
//!
//! ## Context-element → insertion-mode table
//!
//! Per WHATWG HTML §12.4, step 8:
//!
//! | Context element | Mode |
//! |-----------------|------|
//! | `title`, `textarea` | RCDATA (tokenizer switches to RCDATA state; the M4.4.1+ subset keeps rawtext_depth on) |
//! | `style`, `script`, `xmp`, `iframe`, `noembed`, `noframes` | RAWTEXT |
//! | `select` | `InSelect` |
//! | `table`, `tbody`, `tfoot`, `thead` | `InTable` |
//! | `tr`, `td`, `th`, `caption`, `col`, `colgroup` | `InTable` (these are all "table descendants" per the spec) |
//! | everything else | `InBody` |
//!
//! The M4.4.1+ subset unifies RCDATA and RAWTEXT into one
//! "rawtext" mode that keeps appending text to the current top
//! of stack. The `rawtext_depth` counter already in
//! [`super::tree::TreeBuilder`] handles this.

use crate::error::FormatError;
use crate::html::tokeniser::{Mode, Tokeniser};
use crate::html::tree::TreeBuilder;
use crate::token::Token;
use crate::Fragment;

/// Parse `source` as an HTML fragment, given the `context`
/// element (in `context`'s DOM) whose tag name determines the
/// insertion mode.
///
/// The returned `Fragment` owns its own DOM plus a list of
/// top-level nodes produced by the parse. The caller can either
/// inspect the Fragment directly or transplant nodes into
/// another DOM via `frag.dom.append_child(parent, id)`.
pub(crate) fn parse(
    context: &spiral_dom::Dom,
    context_id: spiral_dom::NodeId,
    source: &str,
) -> Result<Fragment, FormatError> {
    let ctx_tag = context
        .get_tag(context_id)
        .ok_or_else(|| FormatError::html_tree(0, 0, "context element has no tag".to_string()))?
        .to_ascii_lowercase();

    let mut builder = TreeBuilder::new_for_fragment(&ctx_tag);
    let mut tokeniser = Tokeniser::new(source);

    // Per WHATWG HTML §12.4 step 8-9, raw-text / script-data
    // context elements cause the tokenizer to switch to
    // RAWTEXT/RCDATA mode BEFORE any tokenisation begins. This
    // is what makes `<title><b>hi</b></title>` parse the `<b>`
    // as text, not as a real element.
    if let Some((mode, end_tag)) = context_to_tokeniser_mode(&ctx_tag) {
        tokeniser.enter_raw_mode(mode, end_tag);
    }

    loop {
        let token = tokeniser.next_token()?;
        if matches!(token, Token::Eof) {
            // Feed the EOF so the builder finalises state.
            builder.feed(&token, &tokeniser)?;
            break;
        }
        builder.feed(&token, &tokeniser)?;
    }

    let ctx_id = builder.fragment_context_id();
    let dom = builder.finish_for_fragment();
    let nodes = fragment_nodes(&dom, ctx_id);

    Ok(Fragment { dom, nodes })
}

/// Map a fragment-context tag name to the tokeniser mode and
/// end-tag scan target, per WHATWG HTML §12.4 step 8-9.
///
/// The M4.4.1+ subset collapses RCDATA and RAWTEXT into the
/// same Mode::Rawtext behaviour — we don't implement character
/// references or scripting-aware `<noscript>`, so the
/// practical difference (RCDATA allows `&amp;` to resolve to
/// `&`) is not observable.
///
/// `tag` must already be lowercased. Returns `None` for
/// non-rawtext contexts (the tokeniser stays in `Normal` mode).
fn context_to_tokeniser_mode(tag: &str) -> Option<(Mode, &'static str)> {
    match tag {
        "title" => Some((Mode::Rawtext, "title")),
        "textarea" => Some((Mode::Rawtext, "textarea")),
        "style" => Some((Mode::Rawtext, "style")),
        "xmp" => Some((Mode::Rawtext, "xmp")),
        "iframe" => Some((Mode::Rawtext, "iframe")),
        "noembed" => Some((Mode::Rawtext, "noembed")),
        "noframes" => Some((Mode::Rawtext, "noframes")),
        "script" => Some((Mode::ScriptData, "script")),
        _ => None,
    }
}

/// Extract the fragment's top-level nodes from the synthetic
/// wrapper DOM.
///
/// The synthetic DOM has the shape:
///   root → <html> → <body> → (synthetic context element)
///                                       → (fragment nodes as children)
///
/// For a `<body>` context the parser does NOT create a synthetic
/// context element — the fragment nodes are direct children of
/// `<body>`. For any other context, the parser creates a synthetic
/// copy of the context element inside `<body>` and pushes it onto
/// the stack so fragment content lands as its children.
///
/// Per the spec, the returned nodes are the children of the
/// synthetic context element (or, for body context, the children
/// of `<body>`). We track the context element's id when we
/// create it (in `parse`) and pass it here.
fn fragment_nodes(
    dom: &spiral_dom::Dom,
    context_element: Option<spiral_dom::NodeId>,
) -> Vec<spiral_dom::NodeId> {
    let Some(context_id) = context_element else {
        // Body context: fragment nodes are direct children of <body>.
        let Some(root_kids) = dom.get_children(dom.root) else {
            return Vec::new();
        };
        let Some(&html) = root_kids
            .iter()
            .find(|&&id| dom.get_tag(id) == Some("html"))
        else {
            return Vec::new();
        };
        let Some(html_kids) = dom.get_children(html) else {
            return Vec::new();
        };
        let Some(&body) = html_kids
            .iter()
            .find(|&&id| dom.get_tag(id) == Some("body"))
        else {
            return Vec::new();
        };
        return dom.get_children(body).unwrap_or_default();
    };

    dom.get_children(context_id).unwrap_or_default()
}
