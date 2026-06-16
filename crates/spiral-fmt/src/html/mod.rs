//! HTML5 parser for Spiral Browser.
//!
//! Public entry point is [`parse`], which takes a `&str` and
//! returns a [`spiral_dom::Dom`]. The implementation is a
//! two-stage pipeline:
//!
//! 1. [`tokeniser::Tokeniser`] — a state machine that converts
//!    the input into a stream of [`crate::token::Token`]s.
//! 2. [`tree::TreeBuilder`] — a simplified insertion-mode machine
//!    that consumes the token stream and constructs a DOM.
//!
//! The implementation is the M4.4.1 minimum-viable subset of
//! the WHATWG HTML5 spec. See the module-level docs on
//! [`tokeniser`] and [`tree`] for the precise scope.

pub mod tokeniser;
pub mod tree;

pub use tokeniser::parse;

/// Maximum nesting depth for the tree builder.
///
/// Limits are hard caps that prevent a hostile input from
/// exhausting the call stack. `512` is the value the WHATWG
/// spec recommends (with adjustments for the well-formed DOM
/// shape).
pub const MAX_NESTING_DEPTH: usize = 512;
