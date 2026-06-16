//! Source span — positions in source text for error reporting and source maps.

/// A half-open range in source text, measured in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Byte offset of the first character.
    pub start: u32,
    /// Byte offset one past the last character.
    pub end: u32,
    /// 1-indexed line number of `start`.
    pub line: u32,
    /// 1-indexed column of `start` (byte-based).
    pub col: u32,
}

impl Span {
    pub fn new(start: u32, end: u32, line: u32, col: u32) -> Self {
        Self {
            start,
            end,
            line,
            col,
        }
    }

    /// A dummy span used when synthesising AST nodes that have no source
    /// position (e.g. desugared constructs).
    pub const DUMMY: Self = Self {
        start: 0,
        end: 0,
        line: 0,
        col: 0,
    };
}
