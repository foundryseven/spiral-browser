//! HTML token types produced by the tokeniser and consumed by the
//! tree builder.
//!
//! The token set is the WHATWG HTML5 standard token set, restricted
//! to the surface needed by the M4.4.1 minimum-viable parser:
//! start tags, end tags, character (text), comments, DOCTYPE,
//! and end-of-file.

use crate::cursor::Position;

/// A single attribute as it appears on a start or end tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Attribute {
    /// Attribute name (lowercased on the public surface; the
    /// tokeniser preserves case for non-foreign content per the
    /// spec).
    pub name: String,
    /// Attribute value (empty string for valueless boolean attrs).
    pub value: String,
}

/// The token kinds the M4.4.1 tokeniser emits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
    /// `<tag>` or `<tag attr="value">` (or the self-closing variant).
    StartTag {
        /// Tag name.
        name: String,
        /// Attributes in source order.
        attributes: Vec<Attribute>,
        /// `True` for `<br/>` and friends; rare in HTML5 content.
        self_closing: bool,
        /// Source position of the opening `<`.
        position: Position,
    },
    /// `</tag>`.
    EndTag {
        /// Tag name.
        name: String,
        /// Source position of the opening `</`.
        position: Position,
    },
    /// Literal text content.
    Character(String),
    /// `<!-- ... -->`.
    Comment {
        /// Comment body.
        text: String,
        /// Source position.
        position: Position,
    },
    /// `<!DOCTYPE name>` and the public/system IDs.
    Doctype {
        /// DOCTYPE name (e.g., `html`).
        name: Option<String>,
        /// Public identifier, if any.
        public_id: Option<String>,
        /// System identifier, if any.
        system_id: Option<String>,
        /// `true` if the parser detected quirks mode from this DOCTYPE.
        quirks: bool,
        /// Source position of the opening `<!`.
        position: Position,
    },
    /// End of input.
    Eof,
}

impl Token {
    /// Source position of the token's first character.
    #[allow(dead_code)]
    pub(crate) fn position(&self) -> Position {
        match self {
            Token::StartTag { position, .. }
            | Token::EndTag { position, .. }
            | Token::Comment { position, .. }
            | Token::Doctype { position, .. } => *position,
            Token::Character(_) | Token::Eof => Position::start(),
        }
    }
}
