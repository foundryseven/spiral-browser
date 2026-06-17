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
//! - `AfterBody`
//! - `AfterAfterBody`
//!
//! These cover the common cases the M4.4.1 test set exercises.
//! The full insertion-mode machine (tables, select, template,
//! foreign content) lands in M5+.

#![allow(clippy::needless_return)]

use crate::cursor::Position;
use crate::error::FormatError;
use crate::token::Token;

/// The insertion mode of the tree builder.
///
/// Each mode determines how a StartTag, EndTag, or Character token
/// is processed.
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
        }
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
                // The head-list rawtext elements (title, style,
                // script, noscript) need to pop themselves off
                // the stack when their end tag arrives; the
                // previous "ignore any other end tag" comment
                // assumed they never appeared, but with the
                // rawtext / script-data tokenisation added
                // (M4.4.1 Item 2) they are real elements with
                // real children.
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
                // Ignore any other end tag inside head.
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
                // Anything else: pop head, transition to body, retry.
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
                // Pop elements until we find one matching the end
                // tag. If none, ignore.
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
                // Parse error: ignore.
                Ok(())
            }
        }
    }

    fn handle_character(
        &mut self,
        text: &str,
        tokeniser: &super::tokeniser::Tokeniser<'_>,
    ) -> Result<(), FormatError> {
        // Inside a raw-text / script-data element, the body is
        // delivered as a single Character token. Treat it as a
        // text append to the current top of stack regardless of
        // insertion mode. This is what keeps `InHead` from
        // re-parenting the body of a `<title>` or `<script>`
        // into `<body>`.
        if self.rawtext_depth > 0 {
            return self.append_text_to_current(text, tokeniser);
        }
        if text.chars().all(|c| c.is_ascii_whitespace()) {
            // Per HTML5, ASCII whitespace handling is mode-specific.
            // For the M4.4.1 minimum we accept whitespace in every
            // mode and append it to the current element (or to
            // `<head>` if that's the current target).
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
                // Non-whitespace before html: parse error, but we
                // recover by jumping to InBody.
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
                    // Whitespace inside `<head>` is fine.
                    return self.append_text_to_current(text, tokeniser);
                }
                // Non-whitespace: close head, transition to body.
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
        if !self.body_created {
            // Only create the body if we're transitioning out of
            // head; calling create_body unconditionally would
            // produce a body in the Initial mode for empty docs.
            if matches!(
                self.mode,
                InsertionMode::InBody | InsertionMode::AfterBody | InsertionMode::AfterAfterBody
            ) {
                self.create_body()?;
            }
        }
        Ok(())
    }

    fn create_html(&mut self) -> Result<(), FormatError> {
        if self.html_created {
            return Ok(());
        }
        let id = self.dom.create_element("html");
        // Append to the document root.
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
        // Find html on the stack and append the head to it.
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
        // Append body to html.
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
        // Ensure the implicit wrappers exist so we always have a
        // legal parent.
        self.flush_implicit()?;
        // Current element: top of the stack, or document root
        // when the stack is empty.
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
        // Per HTML5, adjacent text nodes are merged. The simplest
        // correct approach: if the last child of `parent` is a
        // text node, append to it; otherwise create a new one.
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
        // Kept as a no-op shim so the file remains self-contained.
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

            let formatting_element = match self.active_formatting_elements[formatting_element_idx] {
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
