//! Abstract Syntax Tree nodes for ECMAScript.
//!
//! The AST is the central data structure produced by the parser and consumed
//! by the bytecode compiler (or tree-walking interpreter in early phases).
//! Node types are intentionally concrete — no `Box<dyn Node>` trait objects.
//! Every node carries a `Span` for source-mapping.

pub mod expr;
pub mod span;
pub mod stmt;

pub use expr::Expr;
pub use span::Span;
pub use stmt::Stmt;

/// A complete script or module — the root AST node.
#[derive(Debug, Clone)]
pub struct Program {
    /// The body of the program (a list of statements).
    pub body: Vec<Stmt>,
    /// Whether the source was parsed as a module (allows `import`/`export`).
    pub is_module: bool,
}
