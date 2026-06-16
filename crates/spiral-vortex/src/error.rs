//! Error types for the Vortex JS engine.

use std::fmt;

/// All errors produced by the Vortex engine.
#[derive(Debug, Clone)]
pub enum VortexError {
    /// Lexer error: unexpected character or unterminated string.
    Lex {
        message: String,
        line: u32,
        col: u32,
    },
    /// Parser error: unexpected token, missing semicolon, etc.
    Parse {
        message: String,
        line: u32,
        col: u32,
    },
    /// Runtime TypeError: e.g. calling a non-function, accessing property of
    /// undefined.
    TypeError(String),
    /// ReferenceError: undeclared variable in strict mode.
    ReferenceError(String),
    /// RangeError: invalid array length, stack overflow, etc.
    RangeError(String),
    /// SyntaxError thrown by `eval()` or `new Function()` at runtime.
    SyntaxError(String),
    /// Generic JS-throwable value (user `throw` statement).
    Throw(String),
    /// Internal engine panic — should never surface to user JS.
    Internal(String),
    /// GC allocation failure.
    AllocFailure,
    /// IO error (reading script files, etc.)
    Io(String),
}

impl fmt::Display for VortexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lex { message, line, col } => write!(f, "SyntaxError: {message} at {line}:{col}"),
            Self::Parse { message, line, col } => {
                write!(f, "ParseError: {message} at {line}:{col}")
            }
            Self::TypeError(msg) => write!(f, "TypeError: {msg}"),
            Self::ReferenceError(msg) => write!(f, "ReferenceError: {msg}"),
            Self::RangeError(msg) => write!(f, "RangeError: {msg}"),
            Self::SyntaxError(msg) => write!(f, "SyntaxError: {msg}"),
            Self::Throw(msg) => write!(f, "Uncaught {msg}"),
            Self::Internal(msg) => write!(f, "InternalError: {msg}"),
            Self::AllocFailure => write!(f, "InternalError: allocation failure"),
            Self::Io(msg) => write!(f, "IOError: {msg}"),
        }
    }
}

impl std::error::Error for VortexError {}

/// Convenience alias used throughout the Vortex crate.
pub type VortexResult<T> = Result<T, VortexError>;

impl From<VortexError> for spiral_core::Error {
    fn from(e: VortexError) -> Self {
        spiral_core::Error::JavaScript(e.to_string())
    }
}
