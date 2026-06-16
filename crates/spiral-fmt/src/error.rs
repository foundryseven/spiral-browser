//! Format errors raised by the HTML and CSS parsers.

use thiserror::Error;

/// A format error from the HTML or CSS parser.
///
/// `FormatError` is the single error type for `spiral-fmt`. It
/// carries a human-readable message and an optional 1-based
/// `(line, column)` source position. The `Display` impl is the
/// `Debug + line:col` form; `spiral_core::Error` conversion
/// preserves the location for end-user reports.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum FormatError {
    /// The HTML tokeniser entered a state it could not recover from
    /// (e.g., an unterminated comment, an unterminated DOCTYPE).
    #[error("HTML tokeniser error at {line}:{col}: {message}")]
    HtmlTokeniser {
        /// 1-based line number.
        line: u32,
        /// 1-based column number.
        col: u32,
        /// Human-readable description.
        message: String,
    },

    /// The HTML tree builder rejected a token (e.g., a comment
    /// fragment inside a `<table>`).
    #[error("HTML tree builder error at {line}:{col}: {message}")]
    HtmlTree {
        /// 1-based line number.
        line: u32,
        /// 1-based column number.
        col: u32,
        /// Human-readable description.
        message: String,
    },

    /// The CSS parser raised an error (tokeniser or parser stage).
    /// CSS parsing is a stub in M4.4.1 and returns this for any
    /// non-empty input.
    #[error("CSS parser error: {0}")]
    Css(String),

    /// The input exceeded an internal limit (tokeniser cap,
    /// attribute count, nesting depth). Limits are hard caps
    /// to prevent a hostile input from exhausting memory.
    #[error("Input exceeded limit: {0}")]
    Limit(String),
}

impl FormatError {
    /// Convenience constructor for HTML tokeniser errors.
    #[allow(dead_code)]
    pub(crate) fn html_tokeniser(line: u32, col: u32, message: impl Into<String>) -> Self {
        Self::HtmlTokeniser {
            line,
            col,
            message: message.into(),
        }
    }

    /// Convenience constructor for HTML tree builder errors.
    #[allow(dead_code)]
    pub(crate) fn html_tree(line: u32, col: u32, message: impl Into<String>) -> Self {
        Self::HtmlTree {
            line,
            col,
            message: message.into(),
        }
    }
}
