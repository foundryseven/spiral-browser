//! HTML5 tokeniser (M4.4.1 minimum-viable subset).
//!
//! Implements the WHATWG HTML5 tokeniser state machine, restricted
//! to the surface the M4.4.1 tests require:
//! - start tags, end tags, text, comments, DOCTYPE, EOF
//! - double-, single-, and unquoted attribute values
//! - ASCII whitespace handling per the HTML5 definition
//! - the five named character references (`&amp;`, `&lt;`, `&gt;`,
//!   `&quot;`, `&apos;`)
//! - numeric character references in both decimal (`&#65;`) and
//!   hex (`&#x41;`) forms, with the spec-mandated replacement
//!   table for null, surrogates, out-of-range, and the
//!   0x80..=0x9F Windows-1252 fixup range
//! - `Rawtext` and `ScriptData` modes for `<script>`, `<style>`,
//!   `<textarea>`, `<title>` and the other raw-text / script-data
//!   elements — `<` inside their content is treated as text, not
//!   a tag-open
//!
//! Not in M4.4.1 (deferred to M5+):
//! - CDATA sections, `<![CDATA[...]]>`
//! - foreign content (SVG, MathML)
//! - the full "appropriate end tag token" algorithm in
//!   rawtext / script-data — we use a simple match-on-`</tagname>`
//!   scheme that is correct for the well-formed case
//!
//! ## Algorithm
//! Each call to [`Tokeniser::next_token`] runs the state machine
//! until a token is emitted or EOF is reached. The state is held
//! in [`State`]; buffered start-tag data (name, attributes,
//! self-closing flag) is held on the [`Tokeniser`] and reset after
//! each [`Token::StartTag`] emission. State functions return
//! `Result<Option<Token>>`: `None` means "continue the loop", `Some(t)`
//! means "emit this token and return".

use super::tree::TreeBuilder;
use crate::cursor::{Cursor, Position};
use crate::error::FormatError;
use crate::token::{Attribute, Token};

/// The state of the tokeniser state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum State {
    /// Default state: emit text until a `<` is seen.
    Data,
    /// Just consumed a `<`.
    TagOpen,
    /// Just consumed `</`.
    EndTagOpen,
    /// Inside a start tag's tag-name position.
    TagName,
    /// Whitespace seen inside a tag; next non-ws is an attribute.
    BeforeAttributeName,
    /// Inside an attribute name.
    AttributeName,
    /// Attribute name completed; either `=`, whitespace, or `>`
    /// follows.
    AfterAttributeName,
    /// `=` seen; next is the attribute value (or `>` for valueless).
    BeforeAttributeValue,
    /// Inside `"…"` attribute value.
    AttributeValueDoubleQuoted,
    /// Inside `'…'` attribute value.
    AttributeValueSingleQuoted,
    /// Inside unquoted attribute value (terminated by whitespace
    /// or `>`).
    AttributeValueUnquoted,
    /// `/` seen inside a tag; the next char must be `>` for
    /// self-closing.
    SelfClosingStartTag,
    /// `<!` seen; deciding between DOCTYPE, comment, CDATA, etc.
    MarkupDeclarationOpen,
    /// Inside `<!DOCTYPE …>`.
    Doctype,
    /// `<!--` consumed; the next char determines comment state.
    CommentStart,
    /// Inside `<!-- … -->`.
    Comment,
    /// `--` seen inside a comment; deciding `-->` vs `--!>`.
    CommentEnd,
    /// End of input. The state machine halts.
    Eof,
}

/// The content-mode the tokeniser is currently in.
///
/// Outside the `<script>`, `<style>`, `<textarea>`, `<title>` and
/// related raw-text / script-data elements, the tokeniser is in
/// `Normal` mode and treats `<` as the start of a tag. The tree
/// builder switches the tokeniser to `Rawtext` or `ScriptData`
/// mode for the duration of one of these elements' content, so
/// that a `<` inside the body — e.g. `if (a < b)` in a `<script>`
/// — is delivered as text, not a tag-open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    /// Default mode. Treats `<` as the start of a tag.
    Normal,
    /// Rawtext mode (`<title>`, `<textarea>`, `<style>`, `<xmp>`,
    /// `<iframe>`, `<noembed>`, `<noframes>`, `<noscript>`).
    ///
    /// Inside rawtext, `<` is text until a matching
    /// `</tagname>` is seen, at which point the accumulated
    /// characters are emitted as a single `Character` token and
    /// the end tag is emitted as the next token.
    Rawtext,
    /// Script-data mode (`<script>`).
    ///
    /// Behaves like rawtext but uses `</script>` as the matching
    /// end tag. Per the WHATWG spec, script-data is subtly
    /// different from rawtext in the way it handles `<` (it has
    /// its own sub-states for `</` and `<!--`). For the M4.4.1
    /// minimum we collapse script-data to "text until
    /// `</script>`", which is the behaviour callers actually
    /// observe for well-formed script content.
    ScriptData,
}

/// A stateful HTML5 tokeniser.
///
/// Construct with [`Tokeniser::new`]. Drive with
/// [`Tokeniser::next_token`] until it returns `Token::Eof`.
pub(crate) struct Tokeniser<'a> {
    cursor: Cursor<'a>,
    state: State,
    /// Accumulated start-tag data. Reset after each
    /// [`Token::StartTag`] emission.
    pending_name: String,
    pending_attributes: Vec<Attribute>,
    /// Set to `true` when a self-closing `/` is consumed inside a
    /// start tag. The next `>` causes the [`Token::StartTag`] to
    /// be emitted with `self_closing: true`.
    pending_self_closing: bool,
    /// `true` when the current pending tag is an end tag (`</x>`);
    /// `false` when it is a start tag. Cleared after emission.
    pending_is_end_tag: bool,
    /// The 1-based position of the `<` that opened the current
    /// start tag (or `</` for an end tag). Used to populate
    /// `Token::position()`.
    pending_open_position: Position,
    /// The current content mode. Defaults to `Normal`; the tree
    /// builder switches it to `Rawtext` or `ScriptData` when it
    /// opens a raw-text / script-data element.
    mode: Mode,
    /// The end-tag name we are scanning for in rawtext or
    /// script-data mode (e.g. `"script"` when inside `<script>`).
    /// Lower-case. Empty in `Normal` mode.
    raw_end_tag: &'static str,
}

impl<'a> Tokeniser<'a> {
    /// Construct a new tokeniser over `source`.
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            cursor: Cursor::new(source),
            state: State::Data,
            pending_name: String::new(),
            pending_attributes: Vec::new(),
            pending_self_closing: false,
            pending_is_end_tag: false,
            pending_open_position: Position::start(),
            mode: Mode::Normal,
            raw_end_tag: "",
        }
    }

    /// Switch the tokeniser to rawtext or script-data mode,
    /// scanning for `end_tag` (lower-case, e.g. `"script"`).
    ///
    /// Called by the tree builder immediately after it processes
    /// a start tag for a raw-text or script-data element. The
    /// next call to [`Self::next_token`] will read the body as
    /// text up to the matching end tag, then resume normal
    /// mode.
    pub(crate) fn enter_raw_mode(&mut self, mode: Mode, end_tag: &'static str) {
        self.mode = mode;
        self.raw_end_tag = end_tag;
        self.state = State::Data;
    }

    /// Return to normal mode and forget any pending end-tag scan.
    /// Called by the tree builder after it processes an end tag
    /// in rawtext / script-data.
    pub(crate) fn exit_raw_mode(&mut self) {
        self.mode = Mode::Normal;
        self.raw_end_tag = "";
        self.state = State::Data;
    }

    /// Current 1-based line (for error reporting).
    #[allow(dead_code)]
    pub(crate) fn line(&self) -> u32 {
        self.cursor.line()
    }

    /// Current 1-based column (for error reporting).
    #[allow(dead_code)]
    pub(crate) fn col(&self) -> u32 {
        self.cursor.col()
    }

    /// Drive the state machine until a token is emitted.
    pub(crate) fn next_token(&mut self) -> Result<Token, FormatError> {
        loop {
            let next = match self.mode {
                Mode::Rawtext => self.state_rawtext()?,
                Mode::ScriptData => self.state_script_data()?,
                Mode::Normal => match self.state {
                    State::Data => self.state_data(),
                    State::TagOpen => self.state_tag_open(),
                    State::EndTagOpen => self.state_end_tag_open(),
                    State::TagName => self.state_tag_name(),
                    State::BeforeAttributeName => self.state_before_attribute_name(),
                    State::AttributeName => self.state_attribute_name(),
                    State::AfterAttributeName => self.state_after_attribute_name(),
                    State::BeforeAttributeValue => self.state_before_attribute_value(),
                    State::AttributeValueDoubleQuoted => self.state_attribute_value_dq(),
                    State::AttributeValueSingleQuoted => self.state_attribute_value_sq(),
                    State::AttributeValueUnquoted => self.state_attribute_value_uq(),
                    State::SelfClosingStartTag => self.state_self_closing_start_tag(),
                    State::MarkupDeclarationOpen => self.state_markup_declaration_open(),
                    State::Doctype => self.state_doctype(),
                    State::CommentStart => self.state_comment_start(),
                    State::Comment => self.state_comment(),
                    State::CommentEnd => self.state_comment_end(),
                    State::Eof => return Ok(Token::Eof),
                }?,
            };
            if let Some(token) = next {
                return Ok(token);
            }
        }
    }

    // ------------------------------------------------------------
    // State implementations
    // ------------------------------------------------------------

    fn state_data(&mut self) -> Result<Option<Token>, FormatError> {
        let mut buf = String::new();
        loop {
            match self.cursor.peek_byte() {
                None => {
                    if buf.is_empty() {
                        self.state = State::Eof;
                        return Ok(Some(Token::Eof));
                    }
                    self.state = State::Eof;
                    return Ok(Some(Token::Character(buf)));
                }
                Some(b'<') => {
                    if buf.is_empty() {
                        self.cursor.advance();
                        self.state = State::TagOpen;
                        return Ok(None);
                    }
                    return Ok(Some(Token::Character(buf)));
                }
                Some(b'&') => {
                    if let Some((decoded, consumed)) = try_character_reference(self.cursor.rest()) {
                        buf.push_str(&decoded);
                        self.cursor.advance_n(consumed);
                    } else {
                        buf.push('&');
                        self.cursor.advance();
                    }
                }
                Some(_) => {
                    let ch = self.cursor.current_char().expect("peek_byte returned Some");
                    buf.push(ch);
                    self.cursor.advance();
                }
            }
        }
    }

    fn state_tag_open(&mut self) -> Result<Option<Token>, FormatError> {
        match self.cursor.peek_byte() {
            None => {
                self.state = State::Eof;
                Ok(Some(Token::Character("<".to_string())))
            }
            Some(b'!') => {
                self.cursor.advance();
                self.state = State::MarkupDeclarationOpen;
                Ok(None)
            }
            Some(b'/') => {
                self.cursor.advance();
                self.pending_open_position = self.cursor_position();
                self.state = State::EndTagOpen;
                Ok(None)
            }
            Some(b'a'..=b'z') | Some(b'A'..=b'Z') => {
                self.pending_name.clear();
                self.pending_attributes.clear();
                self.pending_self_closing = false;
                self.pending_open_position = self.cursor_position();
                self.state = State::TagName;
                Ok(None)
            }
            Some(b'?') => {
                // Bogus comment. Per HTML5: parse until `>`. We
                // implement the lenient variant: skip the `?` and
                // return to data.
                self.cursor.advance();
                self.state = State::Data;
                Ok(None)
            }
            Some(_) => {
                self.state = State::Data;
                Ok(Some(Token::Character("<".to_string())))
            }
        }
    }

    fn state_end_tag_open(&mut self) -> Result<Option<Token>, FormatError> {
        match self.cursor.peek_byte() {
            None => {
                self.state = State::Eof;
                Ok(Some(Token::Eof))
            }
            Some(b'a'..=b'z') | Some(b'A'..=b'Z') => {
                self.pending_name.clear();
                self.pending_attributes.clear();
                self.pending_self_closing = false;
                self.pending_is_end_tag = true;
                self.state = State::TagName;
                Ok(None)
            }
            Some(b'>') => {
                // `</>` is a parse error. Recover by emitting
                // the literal `</` as text.
                self.state = State::Data;
                Ok(Some(Token::Character("</".to_string())))
            }
            Some(_) => {
                // `</x` with `x` not a letter. Treat as a bogus
                // comment: skip to `>`.
                self.skip_to_gt();
                self.state = State::Data;
                Ok(None)
            }
        }
    }

    fn state_tag_name(&mut self) -> Result<Option<Token>, FormatError> {
        loop {
            match self.cursor.peek_byte() {
                None => {
                    // Unterminated tag at EOF. Emit whatever we
                    // have; mark EOF.
                    self.state = State::Eof;
                    if self.pending_name.is_empty() {
                        return Ok(Some(Token::Eof));
                    }
                    return Ok(Some(self.take_start_tag()));
                }
                Some(b'\t') | Some(b'\n') | Some(b'\x0c') | Some(b'\r') | Some(b' ') => {
                    self.cursor.advance();
                    if self.pending_name.is_empty() {
                        // `<` then space: not a tag. Emit `<`
                        // and return to Data.
                        self.state = State::Data;
                        return Ok(Some(Token::Character("<".to_string())));
                    }
                    self.state = State::BeforeAttributeName;
                    return Ok(None);
                }
                Some(b'/') => {
                    self.cursor.advance();
                    if self.pending_name.is_empty() {
                        // `<` then `/` then ... already handled in
                        // EndTagOpen. Defensive: emit `<` and let
                        // EndTagOpen process the `/`.
                        self.state = State::EndTagOpen;
                        return Ok(Some(Token::Character("<".to_string())));
                    }
                    self.state = State::SelfClosingStartTag;
                    return Ok(None);
                }
                Some(b'>') => {
                    self.cursor.advance();
                    if self.pending_name.is_empty() {
                        self.state = State::Data;
                        return Ok(Some(Token::Character("<".to_string())));
                    }
                    self.state = State::Data;
                    return Ok(Some(self.take_start_tag()));
                }
                Some(b'\0') => {
                    self.pending_name.push('\u{FFFD}');
                    self.cursor.advance();
                }
                Some(_) => {
                    let ch = self.cursor.current_char().expect("peek_byte returned Some");
                    self.pending_name.push(ch);
                    self.cursor.advance();
                }
            }
        }
    }

    fn state_before_attribute_name(&mut self) -> Result<Option<Token>, FormatError> {
        match self.cursor.peek_byte() {
            None => {
                self.state = State::Eof;
                Ok(Some(self.take_start_tag()))
            }
            Some(b'\t') | Some(b'\n') | Some(b'\x0c') | Some(b'\r') | Some(b' ') => {
                self.cursor.advance();
                Ok(None)
            }
            Some(b'/') => {
                self.cursor.advance();
                self.state = State::SelfClosingStartTag;
                Ok(None)
            }
            Some(b'>') => {
                self.cursor.advance();
                self.state = State::Data;
                Ok(Some(self.take_start_tag()))
            }
            Some(b'=') => {
                // `<tag =`: parse error. Per HTML5, start an
                // attribute with an empty name. We treat this as
                // "no attribute"; the lenient recovery is to skip
                // the `=` and return to AttributeName.
                self.cursor.advance();
                self.state = State::AttributeName;
                Ok(None)
            }
            Some(_) => {
                self.pending_attributes.push(Attribute {
                    name: String::new(),
                    value: String::new(),
                });
                self.state = State::AttributeName;
                Ok(None)
            }
        }
    }

    fn state_attribute_name(&mut self) -> Result<Option<Token>, FormatError> {
        loop {
            let idx = self.pending_attributes.len() - 1;
            match self.cursor.peek_byte() {
                None => {
                    self.state = State::Eof;
                    return Ok(Some(self.take_start_tag()));
                }
                Some(b'\t') | Some(b'\n') | Some(b'\x0c') | Some(b'\r') | Some(b' ')
                | Some(b'/') | Some(b'>') => {
                    let sep = self.cursor.peek_byte().unwrap();
                    self.cursor.advance();
                    match sep {
                        b'>' => {
                            self.state = State::Data;
                            return Ok(Some(self.take_start_tag()));
                        }
                        b'/' => {
                            self.state = State::SelfClosingStartTag;
                            return Ok(None);
                        }
                        _ => {
                            self.state = State::AfterAttributeName;
                            return Ok(None);
                        }
                    }
                }
                Some(b'=') => {
                    self.cursor.advance();
                    self.state = State::BeforeAttributeValue;
                    return Ok(None);
                }
                Some(b'\0') => {
                    self.pending_attributes[idx].name.push('\u{FFFD}');
                    self.cursor.advance();
                }
                Some(_) => {
                    let ch = self.cursor.current_char().expect("peek_byte returned Some");
                    self.pending_attributes[idx].name.push(ch);
                    self.cursor.advance();
                }
            }
        }
    }

    fn state_after_attribute_name(&mut self) -> Result<Option<Token>, FormatError> {
        match self.cursor.peek_byte() {
            None => {
                self.state = State::Eof;
                Ok(Some(self.take_start_tag()))
            }
            Some(b'\t') | Some(b'\n') | Some(b'\x0c') | Some(b'\r') | Some(b' ') => {
                self.cursor.advance();
                Ok(None)
            }
            Some(b'/') => {
                self.cursor.advance();
                self.state = State::SelfClosingStartTag;
                Ok(None)
            }
            Some(b'=') => {
                self.cursor.advance();
                self.state = State::BeforeAttributeValue;
                Ok(None)
            }
            Some(b'>') => {
                self.cursor.advance();
                self.state = State::Data;
                Ok(Some(self.take_start_tag()))
            }
            Some(_) => {
                // Start a new attribute.
                self.pending_attributes.push(Attribute {
                    name: String::new(),
                    value: String::new(),
                });
                self.state = State::AttributeName;
                Ok(None)
            }
        }
    }

    fn state_before_attribute_value(&mut self) -> Result<Option<Token>, FormatError> {
        match self.cursor.peek_byte() {
            None => {
                self.state = State::Eof;
                Ok(Some(self.take_start_tag()))
            }
            Some(b'\t') | Some(b'\n') | Some(b'\x0c') | Some(b'\r') | Some(b' ') => {
                self.cursor.advance();
                Ok(None)
            }
            Some(b'"') => {
                self.cursor.advance();
                self.state = State::AttributeValueDoubleQuoted;
                Ok(None)
            }
            Some(b'\'') => {
                self.cursor.advance();
                self.state = State::AttributeValueSingleQuoted;
                Ok(None)
            }
            Some(b'>') => {
                self.cursor.advance();
                self.state = State::Data;
                Ok(Some(self.take_start_tag()))
            }
            Some(_) => {
                self.state = State::AttributeValueUnquoted;
                Ok(None)
            }
        }
    }

    fn state_attribute_value_dq(&mut self) -> Result<Option<Token>, FormatError> {
        let idx = self.pending_attributes.len() - 1;
        loop {
            match self.cursor.peek_byte() {
                None => {
                    self.state = State::Eof;
                    return Ok(Some(self.take_start_tag()));
                }
                Some(b'"') => {
                    self.cursor.advance();
                    self.state = State::AfterAttributeName;
                    return Ok(None);
                }
                Some(b'&') => {
                    if let Some((decoded, consumed)) = try_character_reference(self.cursor.rest()) {
                        self.pending_attributes[idx].value.push_str(&decoded);
                        self.cursor.advance_n(consumed);
                    } else {
                        self.pending_attributes[idx].value.push('&');
                        self.cursor.advance();
                    }
                }
                Some(b'\0') => {
                    self.pending_attributes[idx].value.push('\u{FFFD}');
                    self.cursor.advance();
                }
                Some(_) => {
                    let ch = self.cursor.current_char().expect("peek_byte returned Some");
                    self.pending_attributes[idx].value.push(ch);
                    self.cursor.advance();
                }
            }
        }
    }

    fn state_attribute_value_sq(&mut self) -> Result<Option<Token>, FormatError> {
        let idx = self.pending_attributes.len() - 1;
        loop {
            match self.cursor.peek_byte() {
                None => {
                    self.state = State::Eof;
                    return Ok(Some(self.take_start_tag()));
                }
                Some(b'\'') => {
                    self.cursor.advance();
                    self.state = State::AfterAttributeName;
                    return Ok(None);
                }
                Some(b'&') => {
                    if let Some((decoded, consumed)) = try_character_reference(self.cursor.rest()) {
                        self.pending_attributes[idx].value.push_str(&decoded);
                        self.cursor.advance_n(consumed);
                    } else {
                        self.pending_attributes[idx].value.push('&');
                        self.cursor.advance();
                    }
                }
                Some(b'\0') => {
                    self.pending_attributes[idx].value.push('\u{FFFD}');
                    self.cursor.advance();
                }
                Some(_) => {
                    let ch = self.cursor.current_char().expect("peek_byte returned Some");
                    self.pending_attributes[idx].value.push(ch);
                    self.cursor.advance();
                }
            }
        }
    }

    fn state_attribute_value_uq(&mut self) -> Result<Option<Token>, FormatError> {
        let idx = self.pending_attributes.len() - 1;
        loop {
            match self.cursor.peek_byte() {
                None => {
                    self.state = State::Eof;
                    return Ok(Some(self.take_start_tag()));
                }
                Some(b'\t') | Some(b'\n') | Some(b'\x0c') | Some(b'\r') | Some(b' ') => {
                    self.cursor.advance();
                    self.state = State::BeforeAttributeName;
                    return Ok(None);
                }
                Some(b'&') => {
                    if let Some((decoded, consumed)) = try_character_reference(self.cursor.rest()) {
                        self.pending_attributes[idx].value.push_str(&decoded);
                        self.cursor.advance_n(consumed);
                    } else {
                        self.pending_attributes[idx].value.push('&');
                        self.cursor.advance();
                    }
                }
                Some(b'>') => {
                    self.cursor.advance();
                    self.state = State::Data;
                    return Ok(Some(self.take_start_tag()));
                }
                Some(b'\0') => {
                    self.pending_attributes[idx].value.push('\u{FFFD}');
                    self.cursor.advance();
                }
                Some(_) => {
                    let ch = self.cursor.current_char().expect("peek_byte returned Some");
                    self.pending_attributes[idx].value.push(ch);
                    self.cursor.advance();
                }
            }
        }
    }

    fn state_self_closing_start_tag(&mut self) -> Result<Option<Token>, FormatError> {
        match self.cursor.peek_byte() {
            Some(b'>') => {
                self.cursor.advance();
                self.pending_self_closing = true;
                self.state = State::Data;
                Ok(Some(self.take_start_tag()))
            }
            None => {
                self.state = State::Eof;
                Ok(Some(self.take_start_tag()))
            }
            Some(_) => {
                // Unexpected character after `/`. Recover by
                // returning to AfterAttributeName.
                self.state = State::AfterAttributeName;
                Ok(None)
            }
        }
    }

    fn state_markup_declaration_open(&mut self) -> Result<Option<Token>, FormatError> {
        if self.cursor.starts_with("--") {
            self.cursor.advance_n(2);
            self.state = State::CommentStart;
            return Ok(None);
        }
        if self.cursor.starts_with("DOCTYPE") {
            self.cursor.advance_n(7);
            self.state = State::Doctype;
            return Ok(None);
        }
        if self.cursor.starts_with("[CDATA[") {
            // CDATA not supported in M4.4.1. Treat as text.
            self.skip_to_eof();
            self.state = State::Eof;
            return Ok(Some(Token::Eof));
        }
        // Bogus comment: skip to `>`.
        self.skip_to_gt();
        self.state = State::Data;
        Ok(None)
    }

    fn state_doctype(&mut self) -> Result<Option<Token>, FormatError> {
        let pos = self.cursor_position();
        self.cursor.skip_ascii_whitespace();
        let mut name: Option<String> = None;
        let mut public_id: Option<String> = None;
        let mut system_id: Option<String> = None;
        let mut quirks = false;
        // Read the DOCTYPE name up to whitespace or `>`.
        let name_start = self.cursor.pos();
        while let Some(b) = self.cursor.peek_byte() {
            if b == b'>' || b.is_ascii_whitespace() {
                break;
            }
            self.cursor.advance();
        }
        if self.cursor.pos() > name_start {
            let raw = self.cursor.slice(name_start, self.cursor.pos());
            name = Some(raw.to_string());
            if !is_known_doctype(raw) {
                quirks = true;
            }
        }
        loop {
            self.cursor.skip_ascii_whitespace();
            match self.cursor.peek_byte() {
                None => {
                    self.state = State::Eof;
                    return Ok(Some(Token::Doctype {
                        name,
                        public_id,
                        system_id,
                        quirks,
                        position: pos,
                    }));
                }
                Some(b'>') => {
                    self.cursor.advance();
                    self.state = State::Data;
                    return Ok(Some(Token::Doctype {
                        name,
                        public_id,
                        system_id,
                        quirks,
                        position: pos,
                    }));
                }
                Some(_) => {
                    if self.cursor.starts_with("PUBLIC") {
                        self.cursor.advance_n(6);
                        self.cursor.skip_ascii_whitespace();
                        if self.cursor.peek_byte() == Some(b'"')
                            || self.cursor.peek_byte() == Some(b'\'')
                        {
                            let quote = self.cursor.peek_byte().unwrap();
                            self.cursor.advance();
                            let start = self.cursor.pos();
                            while let Some(b) = self.cursor.peek_byte() {
                                if b == quote {
                                    break;
                                }
                                self.cursor.advance();
                            }
                            public_id =
                                Some(self.cursor.slice(start, self.cursor.pos()).to_string());
                            if self.cursor.peek_byte() == Some(quote) {
                                self.cursor.advance();
                            }
                        }
                    } else if self.cursor.starts_with("SYSTEM") {
                        self.cursor.advance_n(6);
                        self.cursor.skip_ascii_whitespace();
                        if self.cursor.peek_byte() == Some(b'"')
                            || self.cursor.peek_byte() == Some(b'\'')
                        {
                            let quote = self.cursor.peek_byte().unwrap();
                            self.cursor.advance();
                            let start = self.cursor.pos();
                            while let Some(b) = self.cursor.peek_byte() {
                                if b == quote {
                                    break;
                                }
                                self.cursor.advance();
                            }
                            system_id =
                                Some(self.cursor.slice(start, self.cursor.pos()).to_string());
                            if self.cursor.peek_byte() == Some(quote) {
                                self.cursor.advance();
                            }
                        }
                    } else {
                        // Unknown keyword: skip to whitespace or `>`.
                        while let Some(b) = self.cursor.peek_byte() {
                            if b == b'>' || b.is_ascii_whitespace() {
                                break;
                            }
                            self.cursor.advance();
                        }
                    }
                }
            }
        }
    }

    fn state_comment_start(&mut self) -> Result<Option<Token>, FormatError> {
        match self.cursor.peek_byte() {
            Some(b'-') => {
                self.cursor.advance();
                self.state = State::CommentEnd;
                Ok(None)
            }
            Some(b'>') => {
                self.cursor.advance();
                self.state = State::Data;
                Ok(None)
            }
            None => {
                self.state = State::Eof;
                Ok(Some(Token::Eof))
            }
            Some(_) => {
                self.state = State::Comment;
                Ok(None)
            }
        }
    }

    fn state_comment(&mut self) -> Result<Option<Token>, FormatError> {
        let pos = self.cursor_position();
        let mut text = String::new();
        loop {
            match self.cursor.peek_byte() {
                None => {
                    self.state = State::Eof;
                    return Ok(Some(Token::Comment {
                        text,
                        position: pos,
                    }));
                }
                Some(b'-') => {
                    self.cursor.advance();
                    if self.cursor.peek_byte() == Some(b'-') {
                        self.cursor.advance();
                        self.state = State::CommentEnd;
                        return Ok(Some(Token::Comment {
                            text,
                            position: pos,
                        }));
                    }
                    text.push('-');
                }
                Some(b'<') => {
                    text.push('<');
                    self.cursor.advance();
                }
                Some(b'\0') => {
                    text.push('\u{FFFD}');
                    self.cursor.advance();
                }
                Some(_) => {
                    let ch = self.cursor.current_char().expect("peek_byte returned Some");
                    text.push(ch);
                    self.cursor.advance();
                }
            }
        }
    }

    fn state_comment_end(&mut self) -> Result<Option<Token>, FormatError> {
        match self.cursor.peek_byte() {
            Some(b'>') => {
                self.cursor.advance();
                self.state = State::Data;
                Ok(None)
            }
            Some(b'!') => {
                self.cursor.advance();
                if self.cursor.peek_byte() == Some(b'-') {
                    self.cursor.advance();
                    if self.cursor.peek_byte() == Some(b'>') {
                        self.cursor.advance();
                        self.state = State::Data;
                        return Ok(None);
                    }
                }
                self.state = State::Comment;
                Ok(None)
            }
            Some(b'-') => {
                self.cursor.advance();
                Ok(None)
            }
            None => {
                self.state = State::Eof;
                Ok(Some(Token::Eof))
            }
            Some(_) => {
                self.state = State::Comment;
                Ok(None)
            }
        }
    }

    // ------------------------------------------------------------
    // Helpers
    // ------------------------------------------------------------

    fn cursor_position(&self) -> Position {
        Position {
            line: self.cursor.line(),
            col: self.cursor.col(),
        }
    }

    fn take_start_tag(&mut self) -> Token {
        let name = std::mem::take(&mut self.pending_name);
        let attributes = std::mem::take(&mut self.pending_attributes);
        let self_closing = self.pending_self_closing;
        let is_end = self.pending_is_end_tag;
        self.pending_self_closing = false;
        self.pending_is_end_tag = false;
        if is_end {
            Token::EndTag {
                name,
                position: self.pending_open_position,
            }
        } else {
            Token::StartTag {
                name,
                attributes,
                self_closing,
                position: self.pending_open_position,
            }
        }
    }

    fn skip_to_gt(&mut self) {
        while let Some(b) = self.cursor.peek_byte() {
            if b == b'>' {
                return;
            }
            self.cursor.advance();
        }
    }

    fn skip_to_eof(&mut self) {
        while !self.cursor.eof() {
            self.cursor.advance();
        }
    }

    // ------------------------------------------------------------
    // Rawtext / script-data content (M4.4.1 minimum)
    //
    // These two states collapse the WHATWG spec's "appropriate
    // end tag token" machinery into a single byte scan. The
    // spec defines 9 sub-states per rawtext element and 18 sub-
    // states per script-data element; for the M4.4.1 minimum we
    // observe that, for well-formed content, the only thing that
    // matters is "deliver the body as a single Character token,
    // followed by the matching end tag, then resume normal
    // parsing". Anything more elaborate (e.g. script-data's
    // <!-- ... --> escape handling) is a separate item.
    // ------------------------------------------------------------

    /// Read rawtext (the body of `<title>`, `<textarea>`, etc.)
    /// until the matching end tag is seen.
    ///
    /// Emits a single `Character` token containing everything
    /// before `</end_tag>`, then leaves the cursor positioned at
    /// the start of the end tag. The tree builder, on receiving
    /// the end tag, calls `exit_raw_mode` to put the tokeniser
    /// back in normal mode.
    fn state_rawtext(&mut self) -> Result<Option<Token>, FormatError> {
        self.read_raw_body(Mode::Rawtext)
    }

    /// Read script-data (the body of `<script>`) until the
    /// matching `</script>` is seen. Same shape as
    /// [`Self::state_rawtext`].
    fn state_script_data(&mut self) -> Result<Option<Token>, FormatError> {
        self.read_raw_body(Mode::ScriptData)
    }

    /// Common implementation behind [`Self::state_rawtext`] and
    /// [`Self::state_script_data`].
    ///
    /// Scans the input byte-by-byte, accumulating characters
    /// into a buffer. The only interesting byte is `<`. When
    /// we see `<`, we look ahead to see whether the rest of the
    /// input starts with `</end_tag>` (case-insensitive on the
    /// tag name). If yes, we emit the buffered characters as a
    /// `Character` token, switch back to normal mode, and leave
    /// the cursor at the `<` so the next call to
    /// [`Self::next_token`] re-enters normal mode and parses the
    /// end tag. If no, we append the `<` to the buffer and
    /// continue.
    fn read_raw_body(&mut self, _mode: Mode) -> Result<Option<Token>, FormatError> {
        let mut buf = String::new();
        loop {
            match self.cursor.peek_byte() {
                None => {
                    if buf.is_empty() {
                        return Ok(Some(Token::Eof));
                    }
                    return Ok(Some(Token::Character(buf)));
                }
                Some(b'<') => {
                    if self.at_matching_end_tag() {
                        // Switch back to normal mode *before*
                        // returning so the next call parses
                        // the end tag through the normal
                        // `state_tag_open` / `state_end_tag_open`
                        // path.
                        self.mode = Mode::Normal;
                        self.raw_end_tag = "";
                        self.state = State::Data;
                        return Ok(Some(Token::Character(buf)));
                    }
                    // Not a real end tag — emit `<` as text and
                    // continue. This is the case for `if (a < b)`
                    // inside a `<script>`: the `<b` looks like the
                    // start of an end tag but the next byte is
                    // wrong, so it stays as text.
                    buf.push('<');
                    self.cursor.advance();
                }
                Some(_) => {
                    // Multi-byte UTF-8 is handled by reading the
                    // full char.
                    let ch = self.cursor.current_char().expect("peek_byte returned Some");
                    buf.push(ch);
                    let len = ch.len_utf8();
                    for _ in 1..len {
                        self.cursor.advance();
                    }
                    self.cursor.advance();
                }
            }
        }
    }

    /// True if the cursor is positioned at `</end_tag` (case-
    /// insensitive on the tag name), with `end_tag` being the
    /// raw-mode end tag recorded on the tokeniser.
    fn at_matching_end_tag(&self) -> bool {
        if self.raw_end_tag.is_empty() {
            return false;
        }
        let rest = self.cursor.rest();
        if !rest.starts_with("</") {
            return false;
        }
        let after_lt = &rest[2..];
        if after_lt.len() < self.raw_end_tag.len() + 1 {
            return false;
        }
        // Tag-name bytes must match the configured end tag
        // (case-insensitive).
        if !after_lt
            .as_bytes()
            .iter()
            .take(self.raw_end_tag.len())
            .zip(self.raw_end_tag.as_bytes().iter())
            .all(|(a, b)| a.eq_ignore_ascii_case(b))
        {
            return false;
        }
        // The byte after the tag name must be a terminator: tab,
        // LF, FF, CR, space, slash, or `>`.
        matches!(
            after_lt.as_bytes()[self.raw_end_tag.len()],
            b'\t' | b'\n' | b'\x0c' | b'\r' | b' ' | b'/' | b'>'
        )
    }
}

// ------------------------------------------------------------
// Character reference table (named, M4.4.1 minimum)
// ------------------------------------------------------------

/// Try to parse a named character reference at the start of `input`.
///
/// Returns `Some((decoded, bytes_consumed))` on success, `None` if
/// `input` does not start with a recognised named reference.
///
/// Only the five references that appear in markup are recognised:
/// `&amp;`, `&lt;`, `&gt;`, `&quot;`, `&apos;`.
fn try_named_ref(input: &str) -> Option<(String, usize)> {
    if !input.starts_with('&') {
        return None;
    }
    let after_amp = &input[1..];
    if after_amp.starts_with("amp;") {
        return Some(("&".to_string(), 5));
    }
    if after_amp.starts_with("lt;") {
        return Some(("<".to_string(), 4));
    }
    if after_amp.starts_with("gt;") {
        return Some((">".to_string(), 4));
    }
    if after_amp.starts_with("quot;") {
        return Some(("\"".to_string(), 6));
    }
    if after_amp.starts_with("apos;") {
        return Some(("'".to_string(), 6));
    }
    None
}

/// Try to parse a character reference (named or numeric) at the
/// start of `input`.
///
/// Returns `Some((decoded, bytes_consumed))` on success, `None`
/// if `input` does not start with a recognised reference. The
/// "did not start with a character reference" case includes
/// `&` followed by a non-reference byte — callers in that case
/// should emit a literal `&` and continue. `&#;` (no digits) and
/// `&#x;` (no hex digits) also return `None` for the same
/// reason.
fn try_character_reference(input: &str) -> Option<(String, usize)> {
    if !input.starts_with('&') {
        return None;
    }
    if let Some(result) = try_named_ref(input) {
        return Some(result);
    }
    try_numeric_ref(input)
}

/// Try to parse a numeric character reference (`&#NN;` decimal
/// or `&#xHH;` / `&#XHH;` hex) at the start of `input`.
///
/// Per HTML5 §13.2.5.77 (the "numeric character reference
/// end" state), at most seven digits are consumed (whether
/// decimal or hex). The trailing semicolon is optional in
/// non-attribute contexts and recommended in hex; for the
/// M4.4.1 minimum we accept the form without a semicolon
/// (tolerated as a parse error per spec) since it matches
/// the browser-compatible behaviour that real-world markup
/// relies on.
///
/// Replacement rules applied to the parsed code point, per
/// §13.2.5.78:
///
/// - `0x00`            → U+FFFD
/// - `0xD800..=0xDFFF` → U+FFFD (UTF-16 surrogate halves)
/// - `> 0x10FFFF`      → U+FFFD (outside Unicode range)
/// - `0x80..=0x9F`     → Windows-1252 fixup table
/// - otherwise         → the code point itself
fn try_numeric_ref(input: &str) -> Option<(String, usize)> {
    if !input.starts_with("&#") {
        return None;
    }
    let after_hash = &input[2..];
    let bytes = after_hash.as_bytes();

    let (digits_start, radix) = if bytes.first() == Some(&b'x') || bytes.first() == Some(&b'X') {
        (1, 16)
    } else {
        (0, 10)
    };
    let digits = &bytes[digits_start..];

    // Consume up to 7 digits, with a parse-error-tolerant stop
    // on the first non-digit byte.
    let mut code: u32 = 0;
    let mut count: usize = 0;
    for &b in digits.iter().take(7) {
        let d = match radix {
            10 => match b {
                b'0'..=b'9' => (b - b'0') as u32,
                _ => break,
            },
            16 => match b {
                b'0'..=b'9' => (b - b'0') as u32,
                b'a'..=b'f' => (b - b'a' + 10) as u32,
                b'A'..=b'F' => (b - b'A' + 10) as u32,
                _ => break,
            },
            _ => unreachable!(),
        };
        code = code.saturating_mul(radix as u32).saturating_add(d);
        count += 1;
    }
    if count == 0 {
        // `&#;` or `&#x;` — not a reference.
        return None;
    }

    let mut consumed = 2 + digits_start + count;
    // Optional trailing semicolon.
    if input.as_bytes().get(consumed) == Some(&b';') {
        consumed += 1;
    }

    Some((numeric_replacement(code), consumed))
}

/// Map a parsed numeric code point to the character the
/// WHATWG HTML5 spec says the tokeniser must emit. See
/// [`try_numeric_ref`] for the rules.
fn numeric_replacement(code: u32) -> String {
    // The first three conditions all map to U+FFFD per the
    // spec; we collapse them so the surface looks like one
    // guard rather than three.
    let mapped = if code == 0 || (0xD800..=0xDFFF).contains(&code) || code > 0x10FFFF {
        '\u{FFFD}'
    } else if let Some(c) = windows_1252_fixup(code) {
        c
    } else {
        char::from_u32(code).unwrap_or('\u{FFFD}')
    };
    let mut s = String::new();
    s.push(mapped);
    s
}

/// The Windows-1252 → Unicode mapping for code points 0x80..=0x9F
/// that HTML5 §13.2.5.78 specifies. Code points outside that
/// range return `None` and the caller uses the code point as-is.
fn windows_1252_fixup(code: u32) -> Option<char> {
    let mapped = match code {
        0x80 => '\u{20AC}',              // EURO SIGN
        0x81 => return Some('\u{FFFD}'), // undefined
        0x82 => '\u{201A}',              // SINGLE LOW-9 QUOTATION MARK
        0x83 => '\u{0192}',              // LATIN SMALL LETTER F WITH HOOK
        0x84 => '\u{201E}',              // DOUBLE LOW-9 QUOTATION MARK
        0x85 => '\u{2026}',              // HORIZONTAL ELLIPSIS
        0x86 => '\u{2020}',              // DAGGER
        0x87 => '\u{2021}',              // DOUBLE DAGGER
        0x88 => '\u{02C6}',              // MODIFIER LETTER CIRCUMFLEX ACCENT
        0x89 => '\u{2030}',              // PER MILLE SIGN
        0x8A => '\u{0160}',              // LATIN CAPITAL LETTER S WITH CARON
        0x8B => '\u{2039}',              // SINGLE LEFT-POINTING ANGLE QUOTATION MARK
        0x8C => '\u{0152}',              // LATIN CAPITAL LIGATURE OE
        0x8D => return Some('\u{FFFD}'), // undefined
        0x8E => '\u{017D}',              // LATIN CAPITAL LETTER Z WITH CARON
        0x8F => '\u{FFFD}',              // undefined
        0x90 => '\u{FFFD}',              // undefined
        0x91 => '\u{2018}',              // LEFT SINGLE QUOTATION MARK
        0x92 => '\u{2019}',              // RIGHT SINGLE QUOTATION MARK
        0x93 => '\u{201C}',              // LEFT DOUBLE QUOTATION MARK
        0x94 => '\u{201D}',              // RIGHT DOUBLE QUOTATION MARK
        0x95 => '\u{2022}',              // BULLET
        0x96 => '\u{2013}',              // EN DASH
        0x97 => '\u{2014}',              // EM DASH
        0x98 => '\u{02DC}',              // SMALL TILDE
        0x99 => '\u{2122}',              // TRADE MARK SIGN
        0x9A => '\u{0161}',              // LATIN SMALL LETTER S WITH CARON
        0x9B => '\u{203A}',              // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK
        0x9C => '\u{0153}',              // LATIN SMALL LIGATURE OE
        0x9D => return Some('\u{FFFD}'), // undefined
        0x9E => '\u{017E}',              // LATIN SMALL LETTER Z WITH CARON
        0x9F => '\u{0178}',              // LATIN CAPITAL LETTER Y WITH DIAERESIS
        _ => return None,
    };
    Some(mapped)
}

/// Whether a DOCTYPE name is one of the limited-quirks / no-quirks
/// forms. Anything else puts the parser into quirks mode per
/// the WHATWG HTML5 spec § 13.2.2.1.
fn is_known_doctype(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "html" | "html svg" | "html math" | "html public"
    )
}

/// Drive the tokeniser to completion and feed tokens to a
/// [`TreeBuilder`]. Returns the constructed DOM or a format error.
///
/// The entry point for the public [`crate::parse_html`] facade.
pub(crate) fn tokenise_into(source: &str, builder: &mut TreeBuilder) -> Result<(), FormatError> {
    let mut tokeniser = Tokeniser::new(source);
    loop {
        let token = tokeniser.next_token()?;
        let is_eof = matches!(token, Token::Eof);
        // The tree builder needs to know that a rawtext / script-
        // data element was just opened so it can keep the matching
        // end tag off the auto-close logic. We wire that by
        // switching the tokeniser's mode *after* the start tag has
        // been delivered, but *before* the next call to
        // `next_token` reads raw text.
        if let Token::StartTag { name, .. } = &token {
            if let Some((mode, end_tag)) = raw_mode_for_start_tag(name) {
                tokeniser.enter_raw_mode(mode, end_tag);
            }
        }
        if let Token::EndTag { name, .. } = &token {
            if tokeniser.mode != Mode::Normal && name.eq_ignore_ascii_case(tokeniser.raw_end_tag) {
                tokeniser.exit_raw_mode();
            }
        }
        builder.feed(&token, &tokeniser)?;
        if is_eof {
            return Ok(());
        }
    }
}

/// Return the `(mode, end_tag)` pair to use for an element that
/// triggers rawtext or script-data tokenisation, or `None` if the
/// element does not.
///
/// The list is the WHATWG raw-text / script-data set, restricted
/// to the elements that are not also foreign content. `<plaintext>`
/// is omitted from the M4.4.1 minimum: it never gets a matching
/// end tag, and the M4.4.1 tree builder does not support its
/// behaviour.
fn raw_mode_for_start_tag(name: &str) -> Option<(Mode, &'static str)> {
    match name.to_ascii_lowercase().as_str() {
        "script" => Some((Mode::ScriptData, "script")),
        "style" => Some((Mode::Rawtext, "style")),
        "title" => Some((Mode::Rawtext, "title")),
        "textarea" => Some((Mode::Rawtext, "textarea")),
        "xmp" => Some((Mode::Rawtext, "xmp")),
        "iframe" => Some((Mode::Rawtext, "iframe")),
        "noembed" => Some((Mode::Rawtext, "noembed")),
        "noframes" => Some((Mode::Rawtext, "noframes")),
        // `<noscript>` is rawtext outside of `<head>`; inside
        // `<head>` it is a metadata element. The M4.4.1 tree
        // builder only uses noscript in the head list, so
        // treating it as rawtext here is the safer default.
        "noscript" => Some((Mode::Rawtext, "noscript")),
        _ => None,
    }
}

/// Parse `source` as HTML5 and return a `spiral_dom::Dom`.
///
/// This is the v0.1 minimum-viable parser. It handles the common
/// cases (text, tags, attributes, comments, DOCTYPE) and
/// auto-inserts the implicit `<html><head><body>` wrappers. It
/// is lenient: malformed input is recovered from rather than
/// rejected.
///
/// ## Limitations (M4.4.1)
///
/// - Character references are limited to the five named forms
///   (`&amp;`, `&lt;`, `&gt;`, `&quot;`, `&apos;`).
/// - CDATA sections are treated as text.
/// - Foreign content (SVG, MathML) is treated as ordinary HTML.
/// - Adoption agency, foster parenting, and the template
///   element's content document are deferred to M5+.
///
/// These limitations match the M4.4.1 minimum-viable scope. The
/// 80 additional WPT cases the stretch target requires land in
/// Chunk 4.
pub fn parse(source: &str) -> Result<spiral_dom::Dom, FormatError> {
    let mut builder = TreeBuilder::new();
    tokenise_into(source, &mut builder)?;
    Ok(builder.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect(source: &str) -> Vec<Token> {
        let mut t = Tokeniser::new(source);
        let mut out = Vec::new();
        loop {
            let tok = t.next_token().expect("tokenise");
            let done = matches!(tok, Token::Eof);
            out.push(tok);
            if done {
                break;
            }
        }
        out
    }

    #[test]
    fn simple_div_text_emits_start_text_end() {
        let toks = collect("<div>Hi</div>");
        assert!(matches!(&toks[0], Token::StartTag { name, .. } if name == "div"));
        assert!(matches!(&toks[1], Token::Character(s) if s == "Hi"));
        assert!(matches!(&toks[2], Token::EndTag { name, .. } if name == "div"));
    }

    #[test]
    fn double_quoted_attribute_value() {
        let toks = collect(r#"<a href="https://example.com">x</a>"#);
        if let Token::StartTag {
            name, attributes, ..
        } = &toks[0]
        {
            assert_eq!(name, "a");
            assert_eq!(attributes.len(), 1);
            assert_eq!(attributes[0].name, "href");
            assert_eq!(attributes[0].value, "https://example.com");
        } else {
            panic!("expected StartTag, got {:?}", toks[0]);
        }
    }

    #[test]
    fn doctype_html_is_recognised() {
        let toks = collect("<!DOCTYPE html><html></html>");
        assert!(
            matches!(&toks[0], Token::Doctype { name: Some(n), quirks: false, .. } if n == "html")
        );
    }

    #[test]
    fn doctype_unknown_triggers_quirks() {
        let toks = collect("<!DOCTYPE weird><html></html>");
        assert!(matches!(&toks[0], Token::Doctype { quirks: true, .. }));
    }

    #[test]
    fn comment_is_emitted() {
        let toks = collect("<!-- hello -->");
        assert!(matches!(&toks[0], Token::Comment { text, .. } if text == " hello "));
    }

    #[test]
    fn named_character_references_in_text() {
        let toks = collect("a &amp; b");
        assert!(matches!(&toks[0], Token::Character(s) if s == "a & b"));
    }

    // ------------------------------------------------------------
    // Rawtext and script-data mode tests (M4.4.1 Item 2)
    // ------------------------------------------------------------

    /// Drive the tokeniser in raw mode and return the next
    /// token. The tokeniser starts in normal mode and then has
    /// `enter_raw_mode` called on it, so the tests are
    /// independent of the tree builder wiring.
    fn next_in_raw_mode(source: &str, mode: Mode, end_tag: &'static str) -> Token {
        let mut t = Tokeniser::new(source);
        t.enter_raw_mode(mode, end_tag);
        t.next_token().expect("tokenise")
    }

    #[test]
    fn rawtext_preserves_inner_lt() {
        // `<title>` triggers rawtext. A `<` in the title is
        // not a tag-open.
        let tok = next_in_raw_mode("if a < b then go</title>", Mode::Rawtext, "title");
        assert!(matches!(tok, Token::Character(s) if s == "if a < b then go"));
    }

    #[test]
    fn rawtext_stops_at_matching_end_tag_case_insensitive() {
        let tok = next_in_raw_mode("hello</TITLE>", Mode::Rawtext, "title");
        assert!(matches!(tok, Token::Character(s) if s == "hello"));
    }

    #[test]
    fn script_data_preserves_inner_lt() {
        // The headline case from the audit: `<script>` with
        // `if (a < b)` must not split on the `<`.
        let tok = next_in_raw_mode("if (a < b) {}</script>", Mode::ScriptData, "script");
        assert!(matches!(tok, Token::Character(s) if s == "if (a < b) {}"));
    }

    #[test]
    fn script_data_handles_unterminated_body_at_eof() {
        // No end tag in the source — the whole body becomes a
        // single Character token, followed by EOF on the next
        // call.
        let mut t = Tokeniser::new("if (a < b)");
        t.enter_raw_mode(Mode::ScriptData, "script");
        let tok = t.next_token().expect("tokenise");
        assert!(matches!(tok, Token::Character(s) if s == "if (a < b)"));
        let eof = t.next_token().expect("tokenise");
        assert!(matches!(eof, Token::Eof));
    }

    #[test]
    fn rawtext_end_tag_with_whitespace_after_name() {
        // `<title>foo</title  >` is well-formed; the
        // whitespace terminates the tag name.
        let tok = next_in_raw_mode("foo</title  >", Mode::Rawtext, "title");
        assert!(matches!(tok, Token::Character(s) if s == "foo"));
    }

    #[test]
    fn rawtext_does_not_stop_at_unrelated_end_tag() {
        // `</div>` inside a `<title>` is text, not a terminator.
        let tok = next_in_raw_mode("x</div>y</title>", Mode::Rawtext, "title");
        assert!(matches!(tok, Token::Character(s) if s == "x</div>y"));
    }

    #[test]
    fn raw_mode_round_trip_through_tree_builder() {
        // End-to-end: feed a full document through the tree
        // builder and confirm the text node inside `<script>`
        // contains the `<`.
        let dom = parse("<script>if (a < b) {}</script>").expect("parse");
        let mut found_script = false;
        for (id, _) in dom.descendants(dom.root) {
            if dom.get_tag(id) == Some("script") {
                found_script = true;
                let children = dom.get_children(id).expect("script has children");
                let body: String = children
                    .into_iter()
                    .filter_map(|c| dom.get_text(c).map(|t| t.content.clone()))
                    .collect();
                assert_eq!(body, "if (a < b) {}");
            }
        }
        assert!(found_script, "expected a <script> element in the DOM");
    }

    // ------------------------------------------------------------
    // Numeric character reference tests (M4.4.1 Item 3)
    //
    // WHATWG HTML5 §13.2.5.77-78: `&#NN;` decimal, `&#xHH;` /
    // `&#XHH;` hex, plus the replacement table for null,
    // surrogates, out-of-range, and the 0x80..=0x9F
    // Windows-1252 fixup range.
    // ------------------------------------------------------------

    #[test]
    fn decimal_numeric_ref_in_text() {
        // `&#65;` → 'A'.
        let toks = collect("x&#65;y");
        assert!(matches!(&toks[0], Token::Character(s) if s == "xAy"));
    }

    #[test]
    fn hex_numeric_ref_in_text() {
        // `&#x41;` → 'A'. Upper-case 'X' form is also valid.
        let toks = collect("x&#x41;y");
        assert!(matches!(&toks[0], Token::Character(s) if s == "xAy"));
        let toks = collect("x&#X41;y");
        assert!(matches!(&toks[0], Token::Character(s) if s == "xAy"));
    }

    #[test]
    fn hex_letters_a_to_f() {
        // `&#xA9;` → '©' (decimal 169).
        let toks = collect("&#xA9;");
        assert!(matches!(&toks[0], Token::Character(s) if s == "©"));
    }

    #[test]
    fn numeric_ref_optional_trailing_semicolon() {
        // Both `&#65;` and `&#65` (no semicolon) are accepted
        // for the M4.4.1 minimum; the latter is a tolerated
        // parse error per spec.
        let toks = collect("&#65;");
        assert!(matches!(&toks[0], Token::Character(s) if s == "A"));
        let toks = collect("&#65");
        assert!(matches!(&toks[0], Token::Character(s) if s == "A"));
    }

    #[test]
    fn numeric_ref_consumes_at_most_seven_digits() {
        // `&#0000041;` (decimal) parses to code 41 → ')'.
        // The spec caps consumption at 7 digits; here we
        // consume exactly 7 (the leading zeros plus `41`)
        // and stop on `;`. The result is a single `)` token
        // for the entire reference.
        let toks = collect("&#0000041;");
        assert!(matches!(&toks[0], Token::Character(s) if s == ")"));
    }

    #[test]
    fn numeric_ref_eight_digits_stops_at_seven() {
        // `&#00000041;` has 8 leading zeros before the `;`.
        // The spec caps digit consumption at 7, so we take
        // `0000004` (= 4) and the trailing `1;` is left as
        // text. Since `state_data` accumulates text into a
        // single Character token until `<` or EOF, the
        // resulting token is the whole concatenated string.
        let toks = collect("&#00000041;");
        assert!(matches!(&toks[0], Token::Character(s) if s == "\u{4}1;"));
    }

    #[test]
    fn numeric_ref_null_replaced_with_replacement_char() {
        // `&#0;` → U+FFFD.
        let toks = collect("&#0;");
        assert!(matches!(&toks[0], Token::Character(s) if s == "\u{FFFD}"));
    }

    #[test]
    fn numeric_ref_surrogate_replaced_with_replacement_char() {
        // `&#xD800;` is a UTF-16 high surrogate → U+FFFD.
        let toks = collect("&#xD800;");
        assert!(matches!(&toks[0], Token::Character(s) if s == "\u{FFFD}"));
    }

    #[test]
    fn numeric_ref_out_of_range_replaced_with_replacement_char() {
        // `&#x110000;` is one past the Unicode maximum →
        // U+FFFD.
        let toks = collect("&#x110000;");
        assert!(matches!(&toks[0], Token::Character(s) if s == "\u{FFFD}"));
    }

    #[test]
    fn numeric_ref_windows_1252_fixup() {
        // `&#x80;` → EURO SIGN (U+20AC) per the spec's
        // Windows-1252 table.
        let toks = collect("&#x80;");
        assert!(matches!(&toks[0], Token::Character(s) if s == "\u{20AC}"));
    }

    #[test]
    fn numeric_ref_in_attribute_value() {
        // The same code path serves attribute values.
        let toks = collect(r#"<a href="?x=&#65;">x</a>"#);
        if let Token::StartTag { attributes, .. } = &toks[0] {
            assert_eq!(attributes[0].value, "?x=A");
        } else {
            panic!("expected StartTag, got {:?}", toks[0]);
        }
    }

    #[test]
    fn numeric_ref_no_digits_returns_none() {
        // `&#;` is not a character reference. The wrapper
        // returns `None`, callers emit a literal `&`.
        assert!(try_character_reference("&#;").is_none());
        assert!(try_character_reference("&#x;").is_none());
    }

    #[test]
    fn non_reference_amp_returns_none() {
        // `&foo;` is not one of our five named references; the
        // wrapper returns `None`, callers emit a literal `&`.
        assert!(try_character_reference("&foo;").is_none());
    }

    #[test]
    fn numeric_ref_in_text_unicode_above_bmp() {
        // `&#x1F600;` is a non-BMP code point (😀).
        let toks = collect("&#x1F600;");
        assert!(matches!(&toks[0], Token::Character(s) if s == "😀"));
    }
}
