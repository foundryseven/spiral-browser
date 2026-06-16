//! Byte cursor over the input string.
//!
//! Tracks 1-based line and column for diagnostic messages and
//! exposes a small set of peek/advance/slice helpers. UTF-8
//! boundaries are respected by `current_char`, which returns
//! the next full Unicode scalar value as a `char`.
//!
//! The cursor is the only place that knows about the input
//! string. The tokeniser and tree builder borrow `&mut Cursor`
//! and never see the original `&str`.

/// 1-based source position used in error messages.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Position {
    /// 1-based line number.
    pub line: u32,
    /// 1-based column number.
    pub col: u32,
}

impl Position {
    pub(crate) const fn start() -> Self {
        Self { line: 1, col: 1 }
    }
}

/// Byte cursor with position tracking.
///
/// Construct with [`Cursor::new`]. All `peek_*`/`advance_*`
/// methods are non-consuming; they update `pos`, `line`, `col`
/// in place. `current_char` returns the next Unicode scalar as
/// a `char` (or `None` at end of input).
pub(crate) struct Cursor<'a> {
    /// Underlying input.
    source: &'a str,
    /// Byte offset into `source`.
    pos: usize,
    /// Current 1-based line.
    line: u32,
    /// Current 1-based column.
    col: u32,
}

impl<'a> Cursor<'a> {
    /// Construct a new cursor over `source` starting at line 1, column 1.
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// Whether the cursor has reached end of input.
    pub(crate) fn eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    /// Current 1-based line.
    pub(crate) fn line(&self) -> u32 {
        self.line
    }

    /// Current 1-based column.
    pub(crate) fn col(&self) -> u32 {
        self.col
    }

    /// The remaining unconsumed input.
    pub(crate) fn rest(&self) -> &'a str {
        &self.source[self.pos..]
    }

    /// Current byte offset into the source.
    pub(crate) fn pos(&self) -> usize {
        self.pos
    }

    /// Slice the original source by absolute byte positions.
    ///
    /// Returns the empty string if `start >= end` or if `end`
    /// exceeds the source length. Does not validate UTF-8
    /// boundaries; the caller is responsible for keeping `start`
    /// and `end` on character boundaries.
    pub(crate) fn slice(&self, start: usize, end: usize) -> &'a str {
        if start >= end || end > self.source.len() {
            return "";
        }
        &self.source[start..end]
    }

    /// Peek the next byte without consuming.
    pub(crate) fn peek_byte(&self) -> Option<u8> {
        self.source.as_bytes().get(self.pos).copied()
    }

    /// Peek the next character (full Unicode scalar) without consuming.
    pub(crate) fn current_char(&self) -> Option<char> {
        self.rest().chars().next()
    }

    /// Peek the byte `offset` bytes ahead without consuming.
    #[allow(dead_code)]
    pub(crate) fn peek_byte_at(&self, offset: usize) -> Option<u8> {
        self.source.as_bytes().get(self.pos + offset).copied()
    }

    /// Advance by one ASCII byte, updating line/column counters.
    ///
    /// Line-break semantics:
    /// - `LF` (`\n`): bump the line.
    /// - `CR LF` (`\r\n`): bump the line once.
    /// - A lone `CR` (`\r`): do **not** bump the line; treat as
    ///   a column-only advance. This matches the HTML5 whitespace
    ///   convention where a lone `\r` is whitespace but does not
    ///   start a new line, and keeps the line counter monotonic
    ///   for error reporting.
    ///
    /// All other bytes bump the column. Multi-byte UTF-8
    /// continuation bytes are consumed with the leading byte.
    pub(crate) fn advance(&mut self) {
        match self.peek_byte() {
            Some(b'\n') => {
                self.pos += 1;
                self.line += 1;
                self.col = 1;
            }
            Some(b'\r') => {
                self.pos += 1;
                if self.peek_byte() == Some(b'\n') {
                    // CRLF: consume the LF and bump the line.
                    self.pos += 1;
                    self.line += 1;
                    self.col = 1;
                } else {
                    // Lone CR: column-only advance. The `\r` is
                    // not a line break on its own; matching the
                    // HTML5 whitespace-skip convention.
                    self.col += 1;
                }
            }
            Some(_) => {
                let ch = self.current_char().expect("peek_byte returned Some");
                self.pos += ch.len_utf8();
                self.col += 1;
            }
            None => {}
        }
    }

    /// Advance by `n` bytes; used after a known literal that cannot
    /// contain a newline.
    pub(crate) fn advance_n(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.source.len());
        for _ in 0..n {
            self.advance();
        }
    }

    /// Skip ASCII whitespace per the HTML5 definition
    /// (`U+0009 TAB`, `U+000A LF`, `U+000C FF`, `U+000D CR`, `U+0020 SPACE`).
    pub(crate) fn skip_ascii_whitespace(&mut self) {
        while let Some(b) = self.peek_byte() {
            match b {
                b'\t' | b'\n' | b'\x0c' | b'\r' | b' ' => self.advance(),
                _ => break,
            }
        }
    }

    /// Whether the remaining input starts with `prefix` (case-sensitive).
    pub(crate) fn starts_with(&self, prefix: &str) -> bool {
        self.rest().starts_with(prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_position_is_one_one() {
        let c = Cursor::new("");
        assert_eq!(c.line(), 1);
        assert_eq!(c.col(), 1);
        assert!(c.eof());
    }

    #[test]
    fn advance_simple_ascii() {
        let mut c = Cursor::new("abc");
        assert_eq!(c.current_char(), Some('a'));
        c.advance();
        assert_eq!(c.current_char(), Some('b'));
        assert_eq!(c.col(), 2);
    }

    #[test]
    fn advance_lf_bumps_line() {
        let mut c = Cursor::new("a\nb");
        c.advance(); // a
        c.advance(); // \n
        assert_eq!(c.line(), 2);
        assert_eq!(c.col(), 1);
        assert_eq!(c.current_char(), Some('b'));
    }

    #[test]
    fn advance_crlf_bumps_line_once() {
        let mut c = Cursor::new("a\r\nb");
        c.advance(); // a
        c.advance(); // \r\n
        assert_eq!(c.line(), 2);
        assert_eq!(c.current_char(), Some('b'));
    }

    #[test]
    fn advance_lone_cr_does_not_bump_line() {
        // Per HTML5 whitespace convention, a lone CR (not
        // followed by LF) is whitespace but does not start a
        // new line. The cursor keeps the same line and just
        // advances the column.
        let mut c = Cursor::new("a\rb");
        c.advance(); // a
        c.advance(); // \r
        assert_eq!(c.line(), 1);
        assert_eq!(c.col(), 3);
        assert_eq!(c.current_char(), Some('b'));
    }

    #[test]
    fn advance_utf8_two_byte() {
        // é is two UTF-8 bytes (0xC3 0xA9).
        let mut c = Cursor::new("é");
        assert_eq!(c.current_char(), Some('é'));
        c.advance();
        assert!(c.eof());
    }

    #[test]
    fn skip_whitespace_handles_all_html5_ws_kinds() {
        let mut c = Cursor::new("\t\n\x0c\r a");
        c.skip_ascii_whitespace();
        assert_eq!(c.current_char(), Some('a'));
        assert_eq!(c.line(), 2);
    }

    #[test]
    fn starts_with_literal() {
        let c = Cursor::new("<!DOCTYPE");
        assert!(c.starts_with("<!DOCTYPE"));
        assert!(!c.starts_with("<!doctype"));
    }
}
