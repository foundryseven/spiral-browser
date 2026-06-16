//! Filter engine errors.

use thiserror::Error;

/// Errors from the filter engine.
#[derive(Debug, Error)]
pub enum FilterError {
    #[error("invalid rule syntax: {0}")]
    InvalidSyntax(String),

    #[error("invalid URL pattern: {0}")]
    InvalidPattern(String),

    #[error("invalid CSS selector: {0}")]
    InvalidSelector(String),

    #[error("rule compile error: {0}")]
    CompileError(String),
}
