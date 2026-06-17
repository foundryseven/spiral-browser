//! HTML5 tree builder (M4.4.1 minimum-viable subset).
//!
//! Consumes [`Token`]s from the tokeniser and produces a
//! [`spiral_dom::Dom`]. The builder is a small insertion-mode
//! machine: it tracks the stack of open elements, auto-inserts
//! the implicit `<html><head><body>` wrappers, and recovers
//! from the most common parse errors.
//!
//! The insertion mode set is a strict subset of the WHATWG spec:
//! - `Initial`
//! - `BeforeHtml`
//! - `BeforeHead`
//! - `InHead`
//! - `AfterHead`
//! - `InBody`
//! - `InTable` (Packet 2.8.3 — for foster parenting)
//! - `InTableBody` (Packet 2.8.3 — for foster parenting)
//! - `InRow` (Packet 2.8.3 — for foster parenting)
//! - `InCell` (Packet 2.8.3 — for foster parenting)
//! - `InSelect` (Packet 2.8.3 — for foster parenting)
//! - `AfterBody`
//! - `AfterAfterBody`
#![allow(clippy::needless_return)]
#![allow(clippy::collapsible_if)]


use crate::cursor::Position;
use crate::error::FormatError;
use crate::token::Token;

/// Insertion mode the tree builder is currently in. Per WHATWG
/// HTML §12.2.4.1 the parser transitions between these states as
/// it consumes tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InsertionMode {
    /// Document is empty; the next token starts the parse.
    Initial,
    /// Waiting for `<html>`.
    BeforeHtml,
    /// Waiting for `<head>` (which is auto-created if the first
    /// thing we see isn't head-only).
    BeforeHead,
    /// Inside `<head>`; head-only tags are handled here.
    InHead,
    /// Between `<head>` and `<body>`.
    AfterHead,
    /// Inside `<body>`; the main mode for body content.
    InBody,
    /// After `</body>`, before EOF.
    AfterBody,
    /// After `</html>`, waiting for EOF.
    AfterAfterBody,
    /// Packet 2.8.3 — foster parenting. Active when the stack of
    /// open elements has a `table` in it and the current node is
    /// that `table` or one of its table descendants.
    InTable,
    /// Packet 2.8.3 — active when the current node is a `tbody`,
    /// `thead`, or `tfoot`.
    InTableBody,
    /// Packet 2.8.3 — active when the current node is a `tr`.
    InRow,
    /// Packet 2.8.3 — active when the current node is a `td` or
    /// `th`.
    InCell,
    /// Packet 2.8.3 — active when the current node is a `select`
    /// or one of its descendants. Per spec, `<select>` is one of
    /// the few elements that ALSO trigger foster parenting (in
    /// reverse — non-`<option>`/`<optgroup>` content is kicked
    /// out of the select).
    InSelect,
}

/// Represents an entry in the list of active formatting elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ActiveElement {
    /// A normal DOM node tracked for formatting.
    Element(spiral_dom::NodeId),
    /// A marker boundary (e.g. from table cells or buttons).
    Marker,
}

/// A tree builder that consumes tokens and produces a DOM.
pub(crate) struct TreeBuilder {
    /// The DOM being built. The root (NodeId 0) is always the
    /// document node.
    dom: spiral_dom::Dom,
    /// Stack of open elements. The top of the stack is the
    /// "current" element (the one that receives the next text
    /// or child element).
    stack: Vec<spiral_dom::NodeId>,
    /// List of active formatting elements (AFE).
    active_formatting_elements: Vec<ActiveElement>,
    /// Current insertion mode.
    mode: InsertionMode,
    /// Whether the implicit `<html>` has been created.
    html_created: bool,
    /// Whether the implicit `<head>` has been created.
    head_created: bool,
    /// Whether the implicit `<body>` has been created.
    body_created: bool,
    /// Depth of open raw-text / script-data elements. While this
    /// is non-zero, character tokens are appended to the
    /// current top of the stack regardless of insertion mode.
    /// This is what stops `InHead` from stealing the body of a
    /// `<title>` (or `<script>`, etc.) and re-parenting it
    /// inside `<body>`.
    rawtext_depth: u32,
    /// When [`Self::new_for_fragment`] creates a synthetic
    /// context element (e.g. for a `<div>` or `<select>`
    /// context), this records its NodeId so the caller can
    /// extract the fragment nodes from the right place. For
    /// body-context parses this is `None` (the synthetic body
    /// is the context and the fragment lives as direct
    /// children of body).
    fragment_context_id: Option<spiral_dom::NodeId>,
}

impl TreeBuilder {
    /// Create a new tree builder with an empty DOM and the
    /// `Initial` insertion mode.
    pub(crate) fn new() -> Self {
        Self {
            dom: spiral_dom::Dom::new(),
            stack: Vec::with_capacity(8),
            active_formatting_elements: Vec::new(),
            mode: InsertionMode::Initial,
            html_created: false,
            head_created: false,
            body_created: false,
            rawtext_depth: 0,
            fragment_context_id: None,
        }
    }

    /// Create a tree builder pre-configured for the WHATWG HTML
    /// §12.4 fragment parsing algorithm.
    ///
    /// `context_tag` is the lowercased tag name of the context
    /// element (the element inside which the fragment is being
    /// parsed). It determines:
    ///
    /// 1. The insertion mode the parser starts in.
    /// 2. Whether the tokenizer's rawtext depth is bumped (for
    ///    `title` / `textarea` / `style` / `script` / etc.).
    ///
    /// The builder pre-creates `<html><head><body>` wrappers,
    /// pushes a synthetic copy of the context element onto the
    /// stack (unless the context IS `<body>`, in which case we
    /// reuse the body we just created), and sets the insertion
    /// mode per the spec table. Subsequent token feeds go
    /// through the regular insertion-mode machine.
    pub(crate) fn new_for_fragment(context_tag: &str) -> Self {
        let mut builder = Self {
            dom: spiral_dom::Dom::new(),
            stack: Vec::with_capacity(8),
            active_formatting_elements: Vec::new(),
            mode: InsertionMode::Initial,
            html_created: false,
            head_created: false,
            body_created: false,
            rawtext_depth: 0,
            fragment_context_id: None,
        };

        // Step 4-6 of §12.4: create <html>, push <head>, push <body>.
        // These never exist in the source; they exist solely to give
        // the insertion-mode machine a legal open-elements stack to
        // walk.
        let _ = builder.create_html();
        let _ = builder.create_head();
        let _ = builder.create_body();

        // Step 7-8: handle the context element.
        if context_tag == "body" {
            // The synthetic body IS the context. Push it onto the
            // stack so `flush_implicit` and the token handlers see
            // it as the current element.
            let body_id = builder
                .stack
                .iter()
                .rev()
                .find(|&&id| builder.dom.get_tag(id) == Some("body"))
                .copied();
            if let Some(body_id) = body_id {
                builder.stack.push(body_id);
            }
            builder.mode = InsertionMode::InBody;
        } else {
            // Push a synthetic copy of the context element onto the
            // stack. The parser appends fragment content as children
            // of this element.
            let ctx_id = builder.dom.create_element(context_tag);
            let body_id = builder
                .stack
                .iter()
                .rev()
                .find(|&&id| builder.dom.get_tag(id) == Some("body"))
                .copied();
            if let Some(body_id) = body_id {
                builder
                    .dom
                    .append_child(body_id, ctx_id)
                    .expect("append context element");
            }
            builder.stack.push(ctx_id);
            builder.fragment_context_id = Some(ctx_id);

            builder.mode = context_to_mode(context_tag);

            // Step 8 / 9: rawtext-style context elements bump the
            // rawtext depth so the parser keeps appending text to
            // the context element regardless of the next token.
            if is_rawtext_context(context_tag) {
                builder.rawtext_depth += 1;
            }
        }

        builder
    }

    /// Consume the builder and return the constructed DOM, after
    /// the fragment parse finished. Differs from [`Self::finish`]
    /// only in that the synthetic `<html><head><body>` wrappers
    /// are kept intact so the caller can extract the fragment
    /// nodes from the body element.
    pub(crate) fn finish_for_fragment(mut self) -> spiral_dom::Dom {
        let _ = self.flush_implicit();
        self.dom
    }

    /// The NodeId of the synthetic context element that
    /// [`Self::new_for_fragment`] pushed onto the open-elements
    /// stack. Returns `None` for body-context parses (the
    /// synthetic body IS the context, no extra element is
    /// created).
    pub(crate) fn fragment_context_id(&self) -> Option<spiral_dom::NodeId> {
        self.fragment_context_id
    }

    /// Feed a token to the builder.
    ///
    /// `tokeniser` is used only for `(line, col)` reporting on
    /// error; passing it in avoids duplicating position state on
    /// the builder.
    pub(crate) fn feed(
        &mut self,
        token: &Token,
        tokeniser: &super::tokeniser::Tokeniser<'_>,
    ) -> Result<(), FormatError> {
        match token {
            Token::Eof => {
                // Ensure the implicit wrappers are flushed so the
                // final DOM has `<html><head><body>`.
                self.flush_implicit()?;
                self.mode = InsertionMode::AfterAfterBody;
                Ok(())
            }
            Token::Doctype {
                name,
                quirks,
                position,
                ..
            } => self.handle_doctype(name.as_deref(), *quirks, *position),
            Token::StartTag {
                name,
                attributes,
                position,
                ..
            } => {
                if is_rawtext_element(name) {
                    self.rawtext_depth += 1;
                }
                self.handle_start_tag(name, attributes, *position, tokeniser)
            }
            Token::EndTag { name, position } => {
                if is_rawtext_element(name) && self.rawtext_depth > 0 {
                    self.rawtext_depth -= 1;
                }
                self.handle_end_tag(name, *position, tokeniser)
            }
            Token::Character(text) => self.handle_character(text, tokeniser),
            Token::Comment { text, position } => {
                self.flush_implicit()?;
                let comment_id = self.dom.create_comment(text);
                self.append_to_current(comment_id, *position, tokeniser)
            }
        }
    }

    /// Consume the builder and return the constructed DOM.
    pub(crate) fn finish(mut self) -> spiral_dom::Dom {
        // Drop any unclosed elements; the DOM is whatever the
        // stack produced. This matches the lenient behaviour of
        // browsers on truncated input.
        let _ = self.flush_implicit();
        self.dom
    }

    // ------------------------------------------------------------
    // Token handlers
    // ------------------------------------------------------------

    fn handle_doctype(
        &mut self,
        name: Option<&str>,
        quirks: bool,
        _position: Position,
    ) -> Result<(), FormatError> {
        self.dom.set_quirks_mode(quirks);
        let _ = name; // Recorded for completeness in a future audit pass.
        Ok(())
    }

    fn handle_start_tag(
        &mut self,
        name: &str,
        attributes: &Vec<crate::token::Attribute>,
        position: Position,
        tokeniser: &super::tokeniser::Tokeniser<'_>,
    ) -> Result<(), FormatError> {
        let lower = name.to_ascii_lowercase();
        match self.mode {
            InsertionMode::Initial => {
                // Anything other than whitespace / comment / doctype
                // is a parse error; we recover by jumping to
                // BeforeHtml and re-processing.
                self.mode = InsertionMode::BeforeHtml;
                return self.handle_start_tag(name, attributes, position, tokeniser);
            }
            InsertionMode::BeforeHtml => {
                if lower == "html" {
                    self.create_html()?;
                    self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    self.mode = InsertionMode::BeforeHead;
                    return Ok(());
                }
                // Anything else: auto-create `<html>` and retry.
                self.create_html()?;
                self.mode = InsertionMode::BeforeHead;
                return self.handle_start_tag(name, attributes, position, tokeniser);
            }
            InsertionMode::BeforeHead => {
                if lower == "head" {
                    self.create_head()?;
                    self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    self.mode = InsertionMode::InHead;
                    return Ok(());
                }
                if lower == "html" {
                    // Ignore redundant `<html>` in BeforeHead.
                    return Ok(());
                }
                // Auto-create head and retry.
                self.create_head()?;
                self.mode = InsertionMode::InHead;
                return self.handle_start_tag(name, attributes, position, tokeniser);
            }
            InsertionMode::InHead => {
                match lower.as_str() {
                    "head" => return Ok(()),
                    "html" => return Ok(()),
                    "title" | "base" | "meta" | "link" | "style" | "script" | "noscript" => {
                        self.create_element(&lower)?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                        // Title and similar are void of further tags
                        // in M4.4.1 — we keep them on the stack and
                        // will pop on EndTag. text is still appended.
                        return Ok(());
                    }
                    "body" => {
                        // Close head implicitly; transition to InBody.
                        self.pop_until(|tag| tag == "head");
                        self.mode = InsertionMode::InBody;
                        return self.handle_start_tag(name, attributes, position, tokeniser);
                    }
                    _ => {
                        // Anything else: close head, transition to body.
                        self.pop_until(|tag| tag == "head");
                        self.create_body()?;
                        self.mode = InsertionMode::InBody;
                        return self.handle_start_tag(name, attributes, position, tokeniser);
                    }
                }
            }
            InsertionMode::AfterHead => {
                if lower == "body" {
                    self.create_body()?;
                    self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    self.mode = InsertionMode::InBody;
                    return Ok(());
                }
                if lower == "html" {
                    return Ok(());
                }
                if lower == "head" {
                    return Ok(());
                }
                // Auto-create body and retry.
                self.create_body()?;
                self.mode = InsertionMode::InBody;
                return self.handle_start_tag(name, attributes, position, tokeniser);
            }
            InsertionMode::InBody => {
                if lower == "button" {
                    if self.node_in_scope_by_tag("button") {
                        self.handle_end_tag("button", position, tokeniser)?;
                    }
                    self.reconstruct_active_formatting_elements()?;
                    let _id = self.create_element("button")?;
                    self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    self.active_formatting_elements.push(ActiveElement::Marker);
                    return Ok(());
                }

                // Packet 2.8.3: `<table>` opens a table context.
                // Switch to `InTable` so that subsequent inline /
                // character tokens inside the table get
                // foster-parented per WHATWG §12.2.6.1.
                if lower == "table" {
                    if self.stack_contains("p") {
                        self.pop_until(|tag| tag == "p");
                    }
                    self.reconstruct_active_formatting_elements()?;
                    let _id = self.create_element("table")?;
                    self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    self.active_formatting_elements.push(ActiveElement::Marker);
                    self.mode = InsertionMode::InTable;
                    return Ok(());
                }

                // Packet 2.8.3: `<select>` opens a select context.
                // Switch to `InSelect` so that non-option content
                // gets kicked out per WHATWG §12.2.6.1.
                if lower == "select" {
                    self.reconstruct_active_formatting_elements()?;
                    let _id = self.create_element("select")?;
                    self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    self.mode = InsertionMode::InSelect;
                    return Ok(());
                }

                // Block-level elements: close any open `<p>` first.
                if is_block_level(&lower) && self.stack_contains("p") {
                    self.pop_until(|tag| tag == "p");
                    if self.stack_contains("p") {
                        self.stack.pop();
                    }
                }

                // Self-closing void elements: don't push onto stack.
                if is_void(&lower) {
                    self.reconstruct_active_formatting_elements()?;
                    self.create_element(&lower)?;
                    self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    // Pop the void element immediately.
                    self.stack.pop();
                    return Ok(());
                }

                if is_formatting_element(&lower) {
                    self.reconstruct_active_formatting_elements()?;
                    let id = self.create_element(&lower)?;
                    self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    self.push_active_formatting_element(id);
                    return Ok(());
                }

                self.reconstruct_active_formatting_elements()?;
                self.create_element(&lower)?;
                self.apply_attributes_to_current(attributes, position, tokeniser)?;
                Ok(())
            }
            InsertionMode::AfterBody => {
                if lower == "html" {
                    return Ok(());
                }
                // Parse error: a body-level tag after </body>.
                // Recover by jumping back to InBody and re-processing.
                self.mode = InsertionMode::InBody;
                self.handle_start_tag(name, attributes, position, tokeniser)
            }
            InsertionMode::AfterAfterBody => {
                if lower == "html" {
                    return Ok(());
                }
                // Parse error. Ignore.
                Ok(())
            }
            // ---- Packet 2.8.3: foster parenting start-tags ----
            InsertionMode::InTable => {
                match lower.as_str() {
                    "caption" => {
                        let _id = self.create_element("caption")?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    }
                    "colgroup" => {
                        let _id = self.create_element("colgroup")?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    }
                    "col" => {
                        let _ = self.create_element("colgroup")?;
                        let _id = self.create_element("col")?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                        self.stack.pop(); // self-closing
                    }
                    "thead" | "tbody" | "tfoot" => {
                        let _id = self.create_element(&lower)?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                        self.mode = InsertionMode::InTableBody;
                    }
                    "tr" => {
                        let _ = self.create_element("tbody")?;
                        let _id = self.create_element("tr")?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                        self.mode = InsertionMode::InRow;
                    }
                    "td" | "th" => {
                        let _ = self.create_element("tbody")?;
                        let _ = self.create_element("tr")?;
                        let _id = self.create_element(&lower)?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                        self.mode = InsertionMode::InCell;
                    }
                    "script" | "style" | "template" => {
                        let _id = self.create_element(&lower)?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    }
                    "table" => {
                        // A second <table> while in InTable: close
                        // the current and re-process in InBody.
                        if self.stack_contains("table") {
                            self.pop_until(|tag| tag == "table");
                            self.mode = InsertionMode::InBody;
                            return self.handle_start_tag(
                                name,
                                attributes,
                                position,
                                tokeniser,
                            );
                        }
                        return Ok(());
                    }
                    _ => {
                        // Anything else: foster parent.
                        self.foster_parent(&lower, attributes, position, tokeniser)?;
                    }
                }
                Ok(())
            }
            InsertionMode::InTableBody => {
                match lower.as_str() {
                    "tr" => {
                        let _id = self.create_element("tr")?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                        self.mode = InsertionMode::InRow;
                    }
                    "thead" | "tbody" | "tfoot" => {
                        // A second section before any <tr>: close
                        // the open section and re-process.
                        self.pop_until(|tag| tag == "tbody" || tag == "thead" || tag == "tfoot");
                        let top_tag = self
                            .stack
                            .last()
                            .and_then(|&id| self.dom.get_tag(id));
                        if matches!(top_tag, Some("table")) {
                            return self.handle_start_tag(
                                name,
                                attributes,
                                position,
                                tokeniser,
                            );
                        }
                        return Ok(());
                    }
                    "td" | "th" => {
                        let _ = self.create_element("tr")?;
                        let _id = self.create_element(&lower)?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                        self.mode = InsertionMode::InCell;
                    }
                    "table" => {
                        self.pop_until(|tag| tag == "table");
                        let top_tag = self
                            .stack
                            .last()
                            .and_then(|&id| self.dom.get_tag(id));
                        if matches!(top_tag, Some("html")) {
                            self.mode = InsertionMode::InBody;
                            return self.handle_start_tag(
                                name,
                                attributes,
                                position,
                                tokeniser,
                            );
                        }
                        return Ok(());
                    }
                    _ => {
                        self.foster_parent(&lower, attributes, position, tokeniser)?;
                    }
                }
                Ok(())
            }
            InsertionMode::InRow => {
                match lower.as_str() {
                    "td" | "th" => {
                        let _id = self.create_element(&lower)?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                        self.mode = InsertionMode::InCell;
                    }
                    "tr" | "thead" | "tbody" | "tfoot" => {
                        self.pop_until(|tag| tag == "tr");
                        return self.handle_start_tag(
                            name,
                            attributes,
                            position,
                            tokeniser,
                        );
                    }
                    "table" => {
                        self.pop_until(|tag| tag == "table");
                        let top_tag = self
                            .stack
                            .last()
                            .and_then(|&id| self.dom.get_tag(id));
                        if matches!(top_tag, Some("html")) {
                            self.mode = InsertionMode::InBody;
                            return self.handle_start_tag(
                                name,
                                attributes,
                                position,
                                tokeniser,
                            );
                        }
                        return Ok(());
                    }
                    _ => {
                        self.foster_parent(&lower, attributes, position, tokeniser)?;
                    }
                }
                Ok(())
            }
            InsertionMode::InCell => {
                match lower.as_str() {
                    "td" | "th" | "tr" | "thead" | "tbody" | "tfoot" | "caption"
                    | "colgroup" | "col" | "table" => {
                        self.pop_until(|tag| tag == "td" || tag == "th");
                        return self.handle_start_tag(
                            name,
                            attributes,
                            position,
                            tokeniser,
                        );
                    }
                    _ => {
                        self.foster_parent(&lower, attributes, position, tokeniser)?;
                    }
                }
                Ok(())
            }
            InsertionMode::InSelect => {
                match lower.as_str() {
                    "option" | "optgroup" => {
                        let _id = self.create_element(&lower)?;
                        self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    }
                    _ => {
                        // Anything else: pop the select and re-process
                        // in the parent context.
                        if self.stack_contains("select") {
                            self.pop_until(|tag| tag == "select");
                            if self
                                .stack
                                .last()
                                .map(|&id| self.dom.get_tag(id) == Some("select"))
                                .unwrap_or(false)
                            {
                                self.stack.pop();
                            }
                            self.mode = InsertionMode::InBody;
                            return self.handle_start_tag(
                                name,
                                attributes,
                                position,
                                tokeniser,
                            );
                        }
                        self.mode = InsertionMode::InBody;
                        return self.handle_start_tag(
                            name,
                            attributes,
                            position,
                            tokeniser,
                        );
                    }
                }
                Ok(())
            }
        }
    }

    fn handle_end_tag(
        &mut self,
        name: &str,
        _position: Position,
        _tokeniser: &super::tokeniser::Tokeniser<'_>,
    ) -> Result<(), FormatError> {
        let lower = name.to_ascii_lowercase();
        match self.mode {
            InsertionMode::Initial => Ok(()),
            InsertionMode::BeforeHtml => {
                if lower == "html" {
                    self.create_html()?;
                    self.mode = InsertionMode::BeforeHead;
                }
                Ok(())
            }
            InsertionMode::BeforeHead => {
                if lower == "head" || lower == "body" || lower == "html" || lower == "br" {
                    self.create_head()?;
                    self.mode = InsertionMode::InHead;
                    return self.handle_end_tag(name, _position, _tokeniser);
                }
                Ok(())
            }
            InsertionMode::InHead => {
                if lower == "head" {
                    self.pop_until(|tag| tag == "head");
                    if self
                        .stack
                        .last()
                        .map(|&id| self.dom.get_tag(id) == Some("head"))
                        .unwrap_or(false)
                    {
                        self.stack.pop();
                    }
                    self.mode = InsertionMode::AfterHead;
                    return Ok(());
                }
                if lower == "body" || lower == "html" || lower == "br" {
                    self.pop_until(|tag| tag == "head");
                    if self
                        .stack
                        .last()
                        .map(|&id| self.dom.get_tag(id) == Some("head"))
                        .unwrap_or(false)
                    {
                        self.stack.pop();
                    }
                    self.mode = InsertionMode::AfterHead;
                    return self.handle_end_tag(name, _position, _tokeniser);
                }
                if is_rawtext_element(&lower) {
                    self.pop_until(|tag| tag == lower);
                    if self
                        .stack
                        .last()
                        .map(|&id| self.dom.get_tag(id) == Some(&lower))
                        .unwrap_or(false)
                    {
                        self.stack.pop();
                    }
                    return Ok(());
                }
                Ok(())
            }
            InsertionMode::AfterHead => {
                if lower == "body" || lower == "html" || lower == "br" {
                    self.create_body()?;
                    self.mode = InsertionMode::InBody;
                    return self.handle_end_tag(name, _position, _tokeniser);
                }
                if lower == "head" {
                    return Ok(());
                }
                self.mode = InsertionMode::InBody;
                self.handle_end_tag(name, _position, _tokeniser)
            }
            InsertionMode::InBody => {
                if lower == "body" {
                    self.mode = InsertionMode::AfterBody;
                    return Ok(());
                }
                if lower == "html" {
                    self.mode = InsertionMode::AfterBody;
                    return Ok(());
                }
                if lower == "button" {
                    self.pop_until(|tag| tag == "button");
                    if self
                        .stack
                        .last()
                        .map(|&id| self.dom.get_tag(id) == Some("button"))
                        .unwrap_or(false)
                    {
                        self.stack.pop();
                    }
                    self.clear_up_to_last_marker();
                    return Ok(());
                }
                if is_formatting_element(&lower) {
                    self.run_adoption_agency_algorithm(&lower)?;
                    return Ok(());
                }
                self.pop_until(|tag| tag == lower);
                if self
                    .stack
                    .last()
                    .map(|&id| self.dom.get_tag(id) == Some(&lower))
                    .unwrap_or(false)
                {
                    self.stack.pop();
                }
                Ok(())
            }
            InsertionMode::AfterBody => {
                if lower == "html" {
                    self.mode = InsertionMode::AfterAfterBody;
                }
                Ok(())
            }
            InsertionMode::AfterAfterBody => {
                if lower == "html" {
                    return Ok(());
                }
                Ok(())
            }
            // ---- Packet 2.8.3: foster parenting end-tags ----
            InsertionMode::InTable => {
                if lower == "table" {
                    if self.stack_contains("table") {
                        self.pop_until(|tag| tag == "table");
                        self.reset_table_mode();
                    }
                } else if lower == "br" {
                    self.reconstruct_active_formatting_elements()?;
                    let _ = self.create_element("br")?;
                    self.pop_until(|tag| tag == "table");
                    self.reset_table_mode();
                }
                Ok(())
            }
            InsertionMode::InTableBody => {
                if lower == "tbody" || lower == "thead" || lower == "tfoot" {
                    if self.stack_contains(&lower) {
                        self.pop_until(|tag| tag == lower);
                        self.mode = InsertionMode::InTable;
                    }
                } else if lower == "table"
                    && (self.stack_contains("tbody")
                        || self.stack_contains("thead")
                        || self.stack_contains("tfoot"))
                {
                    self.pop_until(|tag| {
                        tag == "tbody" || tag == "thead" || tag == "tfoot"
                    });
                    self.mode = InsertionMode::InTable;
                    return self.handle_end_tag(name, _position, _tokeniser);
                }
                Ok(())
            }
            InsertionMode::InRow => {
                if lower == "tr" {
                    if self.stack_contains("tr") {
                        self.pop_until(|tag| tag == "tr");
                        self.mode = InsertionMode::InTableBody;
                    }
                } else if lower == "table"
                    || lower == "tbody"
                    || lower == "thead"
                    || lower == "tfoot"
                {
                    if self.stack_contains("tr") {
                        self.pop_until(|tag| tag == "tr");
                        self.mode = InsertionMode::InTableBody;
                        return self.handle_end_tag(name, _position, _tokeniser);
                    }
                }
                Ok(())
            }
            InsertionMode::InCell => {
                if lower == "td" || lower == "th" {
                    if self.stack_contains(&lower) {
                        self.pop_until(|tag| tag == lower);
                        self.mode = InsertionMode::InRow;
                    }
                } else if lower == "tr"
                    || lower == "table"
                    || lower == "tbody"
                    || lower == "thead"
                    || lower == "tfoot"
                {
                    if self.stack_contains("td") || self.stack_contains("th") {
                        self.pop_until(|tag| tag == "td" || tag == "th");
                        self.mode = InsertionMode::InRow;
                        return self.handle_end_tag(name, _position, _tokeniser);
                    }
                }
                Ok(())
            }
            InsertionMode::InSelect => {
                if lower == "select" {
                    if self.stack_contains("select") {
                        self.pop_until(|tag| tag == "select");
                        self.mode = InsertionMode::InBody;
                    }
                } else if lower == "option" || lower == "optgroup" {
                    if self.stack_contains(&lower) {
                        self.pop_until(|tag| tag == lower);
                        if self
                            .stack
                            .last()
                            .map(|&id| self.dom.get_tag(id) == Some(lower.as_str()))
                            .unwrap_or(false)
                        {
                            self.stack.pop();
                        }
                    }
                }
                Ok(())
            }
        }
    }

    fn handle_character(
        &mut self,
        text: &str,
        tokeniser: &super::tokeniser::Tokeniser<'_>,
    ) -> Result<(), FormatError> {
        if self.rawtext_depth > 0 {
            return self.append_text_to_current(text, tokeniser);
        }
        if text.chars().all(|c| c.is_ascii_whitespace()) {
            if matches!(
                self.mode,
                InsertionMode::Initial | InsertionMode::BeforeHtml
            ) {
                return Ok(());
            }
        }
        match self.mode {
            InsertionMode::Initial => {
                if text.chars().all(|c| c.is_ascii_whitespace()) {
                    return Ok(());
                }
                self.flush_implicit()?;
                self.mode = InsertionMode::InBody;
                return self.handle_character(text, tokeniser);
            }
            InsertionMode::BeforeHtml => {
                if text.chars().all(|c| c.is_ascii_whitespace()) {
                    return Ok(());
                }
                self.create_html()?;
                self.mode = InsertionMode::BeforeHead;
                return self.handle_character(text, tokeniser);
            }
            InsertionMode::BeforeHead => {
                if text.chars().all(|c| c.is_ascii_whitespace()) {
                    return Ok(());
                }
                self.create_head()?;
                self.mode = InsertionMode::InHead;
                return self.handle_character(text, tokeniser);
            }
            InsertionMode::InHead => {
                if text.chars().all(|c| c.is_ascii_whitespace()) {
                    return self.append_text_to_current(text, tokeniser);
                }
                self.pop_until(|tag| tag == "head");
                if self
                    .stack
                    .last()
                    .map(|&id| self.dom.get_tag(id) == Some("head"))
                    .unwrap_or(false)
                {
                    self.stack.pop();
                }
                self.create_body()?;
                self.mode = InsertionMode::InBody;
                return self.handle_character(text, tokeniser);
            }
            InsertionMode::AfterHead => {
                if text.chars().all(|c| c.is_ascii_whitespace()) {
                    return self.append_text_to_current(text, tokeniser);
                }
                self.create_body()?;
                self.mode = InsertionMode::InBody;
                return self.handle_character(text, tokeniser);
            }
            InsertionMode::InBody | InsertionMode::AfterBody | InsertionMode::AfterAfterBody => {
                if self.mode == InsertionMode::InBody {
                    self.reconstruct_active_formatting_elements()?;
                }
                self.append_text_to_current(text, tokeniser)
            }
            // ---- Packet 2.8.3: foster parenting characters ----
            InsertionMode::InTable => {
                if text.chars().all(|c| c.is_ascii_whitespace()) {
                    return Ok(());
                }
                self.foster_parent_text(text, tokeniser)?;
                Ok(())
            }
            InsertionMode::InTableBody | InsertionMode::InRow | InsertionMode::InCell => {
                self.append_text_to_current(text, tokeniser)
            }
            InsertionMode::InSelect => {
                self.append_text_to_current(text, tokeniser)
            }
        }
    }

    // ------------------------------------------------------------
    // DOM construction helpers
    // ------------------------------------------------------------

    fn flush_implicit(&mut self) -> Result<(), FormatError> {
        if !self.html_created {
            self.create_html()?;
        }
        if !self.head_created {
            self.create_head()?;
        }
        if !self.body_created
            && matches!(
                self.mode,
                InsertionMode::InBody
                    | InsertionMode::AfterBody
                    | InsertionMode::AfterAfterBody
                    | InsertionMode::InTable
                    | InsertionMode::InTableBody
                    | InsertionMode::InRow
                    | InsertionMode::InCell
                    | InsertionMode::InSelect
            )
        {
            self.create_body()?;
        }
        Ok(())
    }

    fn create_html(&mut self) -> Result<(), FormatError> {
        if self.html_created {
            return Ok(());
        }
        let id = self.dom.create_element("html");
        self.dom
            .append_child(self.dom.root, id)
            .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
        self.stack.push(id);
        self.html_created = true;
        Ok(())
    }

    fn create_head(&mut self) -> Result<(), FormatError> {
        if self.head_created {
            return Ok(());
        }
        if !self.html_created {
            self.create_html()?;
        }
        let html = self
            .stack
            .iter()
            .rev()
            .find(|&&id| self.dom.get_tag(id) == Some("html"))
            .copied()
            .unwrap_or(self.dom.root);
        let id = self.dom.create_element("head");
        self.dom
            .append_child(html, id)
            .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
        self.stack.push(id);
        self.head_created = true;
        Ok(())
    }

    fn create_body(&mut self) -> Result<(), FormatError> {
        if self.body_created {
            return Ok(());
        }
        if !self.html_created {
            self.create_html()?;
        }
        if !self.head_created {
            self.create_head()?;
        }
        let html = self
            .stack
            .iter()
            .rev()
            .find(|&&id| self.dom.get_tag(id) == Some("html"))
            .copied()
            .unwrap_or(self.dom.root);
        let id = self.dom.create_element("body");
        self.dom
            .append_child(html, id)
            .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
        self.stack.push(id);
        self.body_created = true;
        Ok(())
    }

    fn create_element(&mut self, tag: &str) -> Result<spiral_dom::NodeId, FormatError> {
        self.flush_implicit()?;
        let parent = *self.stack.last().unwrap_or(&self.dom.root);
        let id = self.dom.create_element(tag);
        self.dom
            .append_child(parent, id)
            .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
        self.stack.push(id);
        Ok(id)
    }

    fn apply_attributes_to_current(
        &mut self,
        attributes: &[crate::token::Attribute],
        _position: Position,
        _tokeniser: &super::tokeniser::Tokeniser<'_>,
    ) -> Result<(), FormatError> {
        let current = match self.stack.last() {
            Some(&id) => id,
            None => return Ok(()),
        };
        for attr in attributes {
            if attr.name.is_empty() {
                continue;
            }
            if let Err(e) = self.dom.set_attribute(current, &attr.name, &attr.value) {
                return Err(FormatError::html_tree(0, 0, e.to_string()));
            }
        }
        Ok(())
    }

    fn append_to_current(
        &mut self,
        child: spiral_dom::NodeId,
        _position: Position,
        _tokeniser: &super::tokeniser::Tokeniser<'_>,
    ) -> Result<(), FormatError> {
        self.flush_implicit()?;
        let parent = *self.stack.last().unwrap_or(&self.dom.root);
        self.dom
            .append_child(parent, child)
            .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))
    }

    fn append_text_to_current(
        &mut self,
        text: &str,
        _tokeniser: &super::tokeniser::Tokeniser<'_>,
    ) -> Result<(), FormatError> {
        self.flush_implicit()?;
        let parent = *self.stack.last().unwrap_or(&self.dom.root);
        if let Some(children) = self.dom.get_children(parent) {
            if let Some(&last) = children.last() {
                if let Some(existing) = self.dom.get_text_mut(last) {
                    existing.content.push_str(text);
                    return Ok(());
                }
            }
        }
        let id = self.dom.create_text(text);
        self.dom
            .append_child(parent, id)
            .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))
    }

    fn stack_contains(&self, tag: &str) -> bool {
        self.stack
            .iter()
            .any(|&id| self.dom.get_tag(id) == Some(tag))
    }

    fn pop_until<F>(&mut self, pred: F)
    where
        F: Fn(&str) -> bool,
    {
        while let Some(&id) = self.stack.last() {
            if let Some(tag) = self.dom.get_tag(id) {
                if pred(tag) {
                    return;
                }
            }
            self.stack.pop();
        }
    }

    #[allow(dead_code)]
    fn set_quirks_mode(&mut self, _quirks: bool) {
        // Deprecated in favour of `Dom::set_quirks_mode`.
    }

    fn push_active_formatting_element(&mut self, node_id: spiral_dom::NodeId) {
        let tag = match self.dom.get_tag(node_id) {
            Some(t) => t.to_string(),
            None => return,
        };
        let attrs = self
            .dom
            .get_attributes(node_id)
            .map(|attrs| attrs.to_vec())
            .unwrap_or_default();

        let mut identical_indices = Vec::new();
        for (i, entry) in self.active_formatting_elements.iter().enumerate().rev() {
            match entry {
                ActiveElement::Marker => break,
                ActiveElement::Element(id) => {
                    if self.dom.get_tag(*id) == Some(&tag) {
                        let entry_attrs = self.dom.get_attributes(*id);
                        if entry_attrs.map(|a| a == attrs).unwrap_or(false) {
                            identical_indices.push(i);
                        }
                    }
                }
            }
        }

        if identical_indices.len() >= 3 {
            let oldest_idx = *identical_indices.last().unwrap();
            self.active_formatting_elements.remove(oldest_idx);
        }

        self.active_formatting_elements
            .push(ActiveElement::Element(node_id));
    }

    fn node_in_scope(&self, node_id: spiral_dom::NodeId) -> bool {
        let idx = match self.stack.iter().position(|&x| x == node_id) {
            Some(i) => i,
            None => return false,
        };
        for &stack_id in &self.stack[idx..] {
            if let Some(tag) = self.dom.get_tag(stack_id) {
                if matches!(tag, "html" | "table" | "td" | "th" | "button" | "caption")
                    && stack_id != node_id
                {
                    return false;
                }
            }
        }
        true
    }

    fn node_in_scope_by_tag(&self, tag: &str) -> bool {
        let node_id = match self
            .stack
            .iter()
            .rev()
            .find(|&&id| self.dom.get_tag(id) == Some(tag))
        {
            Some(&id) => id,
            None => return false,
        };
        self.node_in_scope(node_id)
    }

    fn clear_up_to_last_marker(&mut self) {
        while let Some(entry) = self.active_formatting_elements.pop() {
            if matches!(entry, ActiveElement::Marker) {
                break;
            }
        }
    }

    fn reconstruct_active_formatting_elements(&mut self) -> Result<(), FormatError> {
        if self.active_formatting_elements.is_empty() {
            return Ok(());
        }

        let last_entry = *self.active_formatting_elements.last().unwrap();
        match last_entry {
            ActiveElement::Marker => return Ok(()),
            ActiveElement::Element(id) => {
                if self.stack.contains(&id) {
                    return Ok(());
                }
            }
        }

        let mut index = self.active_formatting_elements.len() - 1;
        loop {
            if index == 0 {
                break;
            }
            let prev = self.active_formatting_elements[index - 1];
            match prev {
                ActiveElement::Marker => break,
                ActiveElement::Element(id) => {
                    if self.stack.contains(&id) {
                        break;
                    }
                }
            }
            index -= 1;
        }

        let mut start_index = index;
        if let ActiveElement::Element(id) = self.active_formatting_elements[start_index] {
            if self.stack.contains(&id) {
                start_index += 1;
            }
        }

        for i in start_index..self.active_formatting_elements.len() {
            let entry = self.active_formatting_elements[i];
            if let ActiveElement::Element(old_id) = entry {
                let tag = self.dom.get_tag(old_id).unwrap().to_string();
                let attrs = self
                    .dom
                    .get_attributes(old_id)
                    .map(|attrs| attrs.to_vec())
                    .unwrap_or_default();

                let clone_id = self.dom.create_element(&tag);
                for attr in attrs {
                    self.dom
                        .set_attribute(clone_id, &attr.0, &attr.1)
                        .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
                }

                let parent = *self.stack.last().unwrap_or(&self.dom.root);
                self.dom
                    .append_child(parent, clone_id)
                    .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;

                self.active_formatting_elements[i] = ActiveElement::Element(clone_id);
                self.stack.push(clone_id);
            }
        }

        Ok(())
    }

    fn run_adoption_agency_algorithm(&mut self, subject: &str) -> Result<(), FormatError> {
        let mut outer_loop_counter = 0;

        while outer_loop_counter < 8 {
            outer_loop_counter += 1;

            let mut formatting_element_idx = None;
            for (idx, entry) in self.active_formatting_elements.iter().enumerate().rev() {
                match entry {
                    ActiveElement::Marker => break,
                    ActiveElement::Element(id) => {
                        if self.dom.get_tag(*id) == Some(subject) {
                            formatting_element_idx = Some(idx);
                            break;
                        }
                    }
                }
            }

            let formatting_element_idx = match formatting_element_idx {
                None => {
                    self.pop_until(|tag| tag == subject);
                    return Ok(());
                }
                Some(idx) => idx,
            };

            let formatting_element = match self.active_formatting_elements[formatting_element_idx]
            {
                ActiveElement::Element(id) => id,
                _ => unreachable!(),
            };

            if !self.stack.contains(&formatting_element) {
                self.active_formatting_elements
                    .remove(formatting_element_idx);
                return Ok(());
            }

            if !self.node_in_scope(formatting_element) {
                return Ok(());
            }

            let afe_stack_idx = self
                .stack
                .iter()
                .position(|&x| x == formatting_element)
                .unwrap();
            let mut furthest_block = None;
            for &element_id in &self.stack[afe_stack_idx + 1..] {
                if let Some(tag) = self.dom.get_tag(element_id) {
                    if is_special(tag) {
                        furthest_block = Some(element_id);
                        break;
                    }
                }
            }

            let furthest_block = match furthest_block {
                None => {
                    while let Some(pop_id) = self.stack.pop() {
                        if pop_id == formatting_element {
                            break;
                        }
                    }
                    if let Some(afe_idx) = self
                        .active_formatting_elements
                        .iter()
                        .position(|&x| x == ActiveElement::Element(formatting_element))
                    {
                        self.active_formatting_elements.remove(afe_idx);
                    }
                    return Ok(());
                }
                Some(fb) => fb,
            };

            let common_ancestor = self.stack[afe_stack_idx - 1];

            let mut bookmark = self
                .active_formatting_elements
                .iter()
                .position(|&x| x == ActiveElement::Element(formatting_element))
                .unwrap();

            let mut last_node = furthest_block;
            let mut node = furthest_block;
            let mut inner_loop_counter = 0;

            let mut index = self.stack.iter().position(|&x| x == node).unwrap();
            while inner_loop_counter < 3 {
                inner_loop_counter += 1;
                index -= 1;
                node = self.stack[index];

                let node_afe_pos = self
                    .active_formatting_elements
                    .iter()
                    .position(|&x| x == ActiveElement::Element(node));
                if node_afe_pos.is_none() {
                    self.stack.remove(index);
                    continue;
                }

                if node == formatting_element {
                    break;
                }

                let node_afe_idx = node_afe_pos.unwrap();
                if last_node == furthest_block {
                    bookmark = node_afe_idx + 1;
                }

                let clone_id = {
                    let tag = self.dom.get_tag(node).unwrap().to_string();
                    let attrs = self
                        .dom
                        .get_attributes(node)
                        .map(|attrs| attrs.to_vec())
                        .unwrap_or_default();
                    let cid = self.dom.create_element(&tag);
                    for attr in attrs {
                        self.dom
                            .set_attribute(cid, &attr.0, &attr.1)
                            .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
                    }
                    cid
                };

                self.active_formatting_elements[node_afe_idx] = ActiveElement::Element(clone_id);
                self.stack[index] = clone_id;
                node = clone_id;

                if let Some(parent) = self.dom.get_parent(last_node) {
                    self.dom
                        .remove_child(parent, last_node)
                        .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
                }
                self.dom
                    .append_child(node, last_node)
                    .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;

                last_node = node;
            }

            if let Some(parent) = self.dom.get_parent(last_node) {
                self.dom
                    .remove_child(parent, last_node)
                    .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
            }
            self.dom
                .append_child(common_ancestor, last_node)
                .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;

            let clone_id = {
                let tag = self.dom.get_tag(formatting_element).unwrap().to_string();
                let attrs = self
                    .dom
                    .get_attributes(formatting_element)
                    .map(|attrs| attrs.to_vec())
                    .unwrap_or_default();
                let cid = self.dom.create_element(&tag);
                for attr in attrs {
                    self.dom
                        .set_attribute(cid, &attr.0, &attr.1)
                        .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
                }
                cid
            };

            if let Some(children) = self.dom.get_children(furthest_block) {
                for child in children {
                    self.dom
                        .remove_child(furthest_block, child)
                        .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
                    self.dom
                        .append_child(clone_id, child)
                        .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
                }
            }

            self.dom
                .append_child(furthest_block, clone_id)
                .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;

            if let Some(pos) = self
                .active_formatting_elements
                .iter()
                .position(|&x| x == ActiveElement::Element(formatting_element))
            {
                self.active_formatting_elements.remove(pos);
            }
            if bookmark > self.active_formatting_elements.len() {
                bookmark = self.active_formatting_elements.len();
            }
            self.active_formatting_elements
                .insert(bookmark, ActiveElement::Element(clone_id));

            if let Some(pos) = self.stack.iter().position(|&x| x == formatting_element) {
                self.stack.remove(pos);
            }
            let fb_stack_idx = self
                .stack
                .iter()
                .position(|&x| x == furthest_block)
                .unwrap();
            self.stack.insert(fb_stack_idx + 1, clone_id);
        }

        Ok(())
    }

    // ------------------------------------------------------------
    // Packet 2.8.3 — foster parenting
    // ------------------------------------------------------------
    //
    // The "foster parent" of an element is the parent of the
    // most-recent `table` in the stack of open elements. Per
    // WHATWG §12.2.6.1, when an inline tag or non-whitespace text
    // arrives inside a table context, we:
    //   1. Find the most recent `table` ancestor.
    //   2. Insert the orphan as a SIBLING of that table — i.e.
    //      append it to the table's parent, just before the table.
    //   3. The orphan is NOT pushed onto the open-elements stack;
    //      its end-tag is a parse error per spec.

    /// Foster parent an element. Called by the `InTable`,
    /// `InTableBody`, `InRow`, `InCell`, and `InSelect` mode arms
    /// for any tag that is not a valid table child.
    fn foster_parent(
        &mut self,
        tag: &str,
        attributes: &[crate::token::Attribute],
        position: Position,
        tokeniser: &super::tokeniser::Tokeniser<'_>,
    ) -> Result<(), FormatError> {
        self.flush_implicit()?;

        let id = self.dom.create_element(tag);

        // The foster target is the parent of the most-recent
        // table on the open-elements stack. If there is no table
        // (e.g. in InSelect, or InTable with no table on the
        // stack — a spec edge case), fall back to the top of
        // stack.
        let table_idx = self
            .stack
            .iter()
            .rposition(|&sid| self.dom.get_tag(sid) == Some("table"));

        if let Some(idx) = table_idx {
            let foster_target = if idx == 0 {
                self.dom.root
            } else {
                self.stack[idx - 1]
            };
            // Insert just BEFORE the table in the foster target's
            // children list, if the table is actually a child of
            // the foster target. Otherwise, fall back to plain
            // append.
            let table_id = self.stack[idx];
            if let Some(children) = self.dom.get_children(foster_target) {
                if let Some(pos) = children.iter().position(|&c| c == table_id) {
                    self.dom
                        .insert_child(foster_target, pos, id)
                        .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
                } else {
                    self.dom
                        .append_child(foster_target, id)
                        .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
                }
            } else {
                self.dom
                    .append_child(foster_target, id)
                    .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
            }
        } else {
            // No table in stack (e.g. in InSelect). Foster to
            // current top of stack.
            let parent = *self.stack.last().unwrap_or(&self.dom.root);
            self.dom
                .append_child(parent, id)
                .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
        }

        // If the new element is a formatting element, push it on
        // the AFE list.
        if is_formatting_element(tag) {
            self.push_active_formatting_element(id);
        }

        // Apply attributes directly to the new element (it is not
        // on the open-elements stack, so we cannot use
        // `apply_attributes_to_current`).
        for attr in attributes {
            if attr.name.is_empty() {
                continue;
            }
            self.dom
                .set_attribute(id, &attr.name, &attr.value)
                .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
        }
        let _ = position;
        let _ = tokeniser;

        // Per spec, the foster-parented element IS pushed onto
        // the open-elements stack so that further content (text,
        // nested inline) goes inside it. The spec then continues
        // processing tokens in InBody mode (because the foster
        // parent placed the orphan outside the table).
        self.stack.push(id);
        self.mode = InsertionMode::InBody;

        Ok(())
    }

    /// Foster parent a text node. Same algorithm as the element
    /// case but creates a text node.
    fn foster_parent_text(
        &mut self,
        text: &str,
        _tokeniser: &super::tokeniser::Tokeniser<'_>,
    ) -> Result<(), FormatError> {
        self.flush_implicit()?;

        let id = self.dom.create_text(text);

        let table_idx = self
            .stack
            .iter()
            .rposition(|&sid| self.dom.get_tag(sid) == Some("table"));

        if let Some(idx) = table_idx {
            let foster_target = if idx == 0 {
                self.dom.root
            } else {
                self.stack[idx - 1]
            };
            let table_id = self.stack[idx];
            if let Some(children) = self.dom.get_children(foster_target) {
                if let Some(pos) = children.iter().position(|&c| c == table_id) {
                    self.dom
                        .insert_child(foster_target, pos, id)
                        .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
                } else {
                    self.dom
                        .append_child(foster_target, id)
                        .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
                }
            } else {
                self.dom
                    .append_child(foster_target, id)
                    .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
            }
        } else {
            let parent = *self.stack.last().unwrap_or(&self.dom.root);
            self.dom
                .append_child(parent, id)
                .map_err(|e| FormatError::html_tree(0, 0, e.to_string()))?;
        }
        Ok(())
    }

    /// After `</table>` (or a parse-error pop that crosses a
    /// table boundary), reset the insertion mode to whatever is
    /// appropriate given the current state of the open-elements
    /// stack. Per WHATWG §12.2.4.1.
    fn reset_table_mode(&mut self) {
        for &id in self.stack.iter().rev() {
            match self.dom.get_tag(id) {
                Some("select") => {
                    self.mode = InsertionMode::InSelect;
                    return;
                }
                Some("td") | Some("th") => {
                    self.mode = InsertionMode::InCell;
                    return;
                }
                Some("tr") => {
                    self.mode = InsertionMode::InRow;
                    return;
                }
                Some("tbody") | Some("thead") | Some("tfoot") => {
                    self.mode = InsertionMode::InTableBody;
                    return;
                }
                Some("table") => {
                    self.mode = InsertionMode::InTable;
                    return;
                }
                _ => continue,
            }
        }
        self.mode = InsertionMode::InBody;
    }
}

/// Whether a tag name is one of the raw-text / script-data
/// elements (`<script>`, `<style>`, `<title>`, `<textarea>` and
/// friends). The tokeniser switches to rawtext or script-data
/// mode for these, and the tree builder keeps their body as
/// text rather than routing it through the normal insertion-
/// mode machine.
///
/// Per the WHATWG spec this is the union of the "raw text" and
/// "script data" sets, restricted to the elements that are not
/// also foreign content. `<plaintext>` is omitted from the
/// M4.4.1 minimum because it has no end tag.
fn is_rawtext_element(tag: &str) -> bool {
    let lower = tag.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "script"
            | "style"
            | "title"
            | "textarea"
            | "xmp"
            | "iframe"
            | "noembed"
            | "noframes"
            | "noscript"
    )
}

/// Map a fragment-context element to its insertion mode per
/// WHATWG HTML §12.4 step 8.
///
/// `tag` must already be lowercased.
fn context_to_mode(tag: &str) -> InsertionMode {
    match tag {
        "select" => InsertionMode::InSelect,
        "table" | "tbody" | "tfoot" | "thead" | "tr" | "td" | "th" | "caption" | "col"
        | "colgroup" => InsertionMode::InTable,
        _ => InsertionMode::InBody,
    }
}

/// Whether a fragment-context tag name requires the tokenizer's
/// rawtext state (so the parser keeps appending text to the
/// context element regardless of the next token). This is the
/// union of the RCDATA and RAWTEXT sets from §12.4 step 8
/// (RCDATA: `title`, `textarea`) and step 9 (RAWTEXT:
/// `style`, `script`, `xmp`, `iframe`, `noembed`, `noframes`).
/// `noscript` and `plaintext` are deliberately omitted from
/// the M4.4.1+ subset — they have spec-defined quirks (scripting
/// toggle for `<noscript>`, no-end-tag for `<plaintext>`) that
/// we do not implement.
///
/// `tag` must already be lowercased.
fn is_rawtext_context(tag: &str) -> bool {
    matches!(
        tag,
        "title"
            | "textarea"
            | "style"
            | "script"
            | "xmp"
            | "iframe"
            | "noembed"
            | "noframes"
    )
}

/// Whether a tag name is one of the HTML5 void elements (which
/// have no end tag and cannot contain children).
fn is_void(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "source"
            | "track"
            | "wbr"
    )
}

/// Whether a tag name is block-level enough to close a `<p>` when
/// it appears.
///
/// The M4.4.1 minimum is a small subset; the full list is in
/// HTML5 § 4.4.5 and lands in M5+.
fn is_block_level(tag: &str) -> bool {
    matches!(
        tag,
        "address"
            | "article"
            | "aside"
            | "blockquote"
            | "details"
            | "dialog"
            | "dd"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "hgroup"
            | "hr"
            | "li"
            | "main"
            | "nav"
            | "ol"
            | "p"
            | "pre"
            | "section"
            | "table"
            | "ul"
    )
}

/// Whether a tag is a formatting element.
fn is_formatting_element(tag: &str) -> bool {
    matches!(
        tag,
        "a" | "b"
            | "big"
            | "code"
            | "em"
            | "font"
            | "i"
            | "nobr"
            | "s"
            | "small"
            | "strike"
            | "strong"
            | "tt"
            | "u"
    )
}

/// Whether a tag belongs to the "special" category in WHATWG HTML.
fn is_special(tag: &str) -> bool {
    matches!(
        tag,
        "address"
            | "applet"
            | "area"
            | "article"
            | "aside"
            | "base"
            | "basefont"
            | "bgsound"
            | "blockquote"
            | "body"
            | "br"
            | "button"
            | "caption"
            | "center"
            | "col"
            | "colgroup"
            | "dd"
            | "details"
            | "dir"
            | "div"
            | "dl"
            | "dt"
            | "embed"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "frame"
            | "frameset"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "head"
            | "header"
            | "hgroup"
            | "hr"
            | "html"
            | "iframe"
            | "img"
            | "input"
            | "keygen"
            | "li"
            | "link"
            | "listing"
            | "main"
            | "marquee"
            | "menu"
            | "meta"
            | "nav"
            | "noembed"
            | "noframes"
            | "noscript"
            | "object"
            | "ol"
            | "p"
            | "param"
            | "plaintext"
            | "pre"
            | "script"
            | "section"
            | "select"
            | "source"
            | "style"
            | "summary"
            | "table"
            | "tbody"
            | "td"
            | "template"
            | "textarea"
            | "tfoot"
            | "th"
            | "thead"
            | "title"
            | "tr"
            | "track"
            | "ul"
            | "wbr"
            | "xmp"
    )
}
