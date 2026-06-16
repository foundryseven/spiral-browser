//! Byte-level cursor for navigating source text.
//!
//! This module is intentionally thin — the `Lexer` struct in `mod.rs` owns
//! the same state directly. The `Cursor` type exists as an alternative
//! entry-point for future streaming/iterator-based lexing and for unit
//! tests that need to drive the source position manually.

/// A lightweight cursor over a UTF-8 source string.
pub struct Cursor<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    /// Create a cursor positioned at the start of the source.
    pub fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }

    /// Current byte offset.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Whether the cursor has reached the end of the source.
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    /// The character at the current position, or `'\0'` at EOF.
    #[inline]
    pub fn current(&self) -> char {
        self.source[self.pos..].chars().next().unwrap_or('\0')
    }

    /// Peek at the next character without advancing.
    #[inline]
    pub fn peek(&self) -> char {
        let mut chars = self.source[self.pos..].chars();
        chars.next();
        chars.next().unwrap_or('\0')
    }

    /// Advance past the current character and return it.
    pub fn advance(&mut self) -> char {
        let ch = self.current();
        self.pos += ch.len_utf8();
        ch
    }

    /// Slice the source between two byte offsets.
    pub fn slice(&self, from: usize, to: usize) -> &str {
        &self.source[from..to]
    }

    /// Remaining source from the current position.
    pub fn rest(&self) -> &str {
        &self.source[self.pos..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_navigation() {
        let mut c = Cursor::new("abc");
        assert_eq!(c.current(), 'a');
        assert_eq!(c.advance(), 'a');
        assert_eq!(c.current(), 'b');
        assert_eq!(c.peek(), 'c');
        assert!(!c.is_eof());
    }

    #[test]
    fn test_eof() {
        let mut c = Cursor::new("a");
        c.advance();
        assert!(c.is_eof());
        assert_eq!(c.current(), '\0');
    }
}
