//! JavaScript value representation.
//!
//! Vortex uses a tagged-value representation for JS values. In Phase 1
//! (tree-walking interpreter) this is a simple enum. The bytecode VM in
//! Phase 2 will switch to NaN-boxing for performance.

pub mod jsvalue;
pub mod number;
pub mod object;
pub mod string;

pub use jsvalue::JsValue;
