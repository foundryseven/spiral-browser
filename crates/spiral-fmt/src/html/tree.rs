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

/// A tree builder that consumes tokens and produces a DOM.
pub(crate) struct TreeBuilder {
    /// The DOM being built. The root (NodeId 0) is always the
    /// document node.
    dom: spiral_dom::Dom,
    /// Stack of open elements. The top of the stack is the
    /// "current" element (the one that receives the next text
    /// or child element).
    stack: Vec<spiral_dom::NodeId>,
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
                // Block-level elements: close any open `<p>` first.
                if is_block_level(&lower) && self.stack_contains("p") {
                    self.pop_until(|tag| tag == "p");
                }
                // Self-closing void elements: don't push onto stack.
                if is_void(&lower) {
                    self.create_element(&lower)?;
                    self.apply_attributes_to_current(attributes, position, tokeniser)?;
                    // Pop the void element immediately.
                    self.stack.pop();
                    return Ok(());
                }
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
                    self.mode = InsertionMode::AfterHead;
                    return Ok(());
                }
                if lower == "body" || lower == "html" || lower == "br" {
                    self.pop_until(|tag| tag == "head");
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
                // Pop elements until we find one matching the end
                // tag. If none, ignore.
                self.pop_until(|tag| tag == lower);
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
