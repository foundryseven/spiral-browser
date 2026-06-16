//! Bytecode virtual machine for the Vortex JavaScript engine.
//!
//! Phase 1 ships with a **tree-walking interpreter** over the AST.
//! Phase 2 adds a **bytecode compiler** + **stack-based VM** that
//! compiles AST to bytecode and interprets it — roughly 5-10× faster
//! than tree-walking, and the foundation for a future baseline JIT.
//!
//! The module structure:
//!
//! - `interpreter` — the Phase 1 tree-walking evaluator.
//! - Future: `bytecode` — bytecode instruction set + compiler.
//! - Future: `stack_vm` — the bytecode interpreter.
//! - Future: `jit` — Cranelift-based baseline JIT.

pub mod interpreter;
