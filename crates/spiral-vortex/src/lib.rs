//! Spiral Browser — Vortex JavaScript Engine
//!
//! Vortex is Spiral's **from-scratch** JavaScript engine, written entirely in
//! safe Rust. It implements ECMAScript (ES2015+) from the ground up:
//! lexer, parser, AST, bytecode compiler, interpreter, mark-sweep GC,
//! and (future) a baseline JIT — no QuickJS, no V8, no Boa.
//!
//! # Architecture
//!
//! ```text
//!   source ──▶ Lexer ──▶ Tokens ──▶ Parser ──▶ AST
//!                                                 │
//!                                           BytecodeCompiler
//!                                                 │
//!                                              Bytecode
//!                                                 │
//!                                           VM (interpreter)
//!                                                 │
//!                                            Builtins / GC
//!                                                 │
//!                                       DOM bindings / Event loop
//! ```
//!
//! # V8 Oracle (CI compliance)
//!
//! When compiled with `--features v8`, the crate also exposes a thin V8
//! wrapper (`crate::v8`) backed by Google's V8 via `rusty_v8`. This is
//! **not** the production engine — it exists only as a CI compliance oracle:
//! the test harness runs the same JS snippets through both Vortex and V8
//! and compares outputs. Default builds have zero V8 dependencies.

pub mod ast;
pub mod builtins;
pub mod dom_bindings;
pub mod error;
pub mod event_loop;
pub mod gc;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod value;
pub mod vm;

#[cfg(feature = "v8")]
pub mod v8;

pub use error::{VortexError, VortexResult};
pub use runtime::Vortex;
pub use value::JsValue;

/// One-shot Vortex entry point: lex, parse, and run a script.
///
/// This is the canonical packet-1.6.2 surface. Use it when the caller
/// does not need to keep a [`Vortex`] runtime alive between calls (e.g.
/// throwaway scripts, tests, the Phase-1 `spiral_context::Context`
/// binding). The returned [`JsValue`] is the value of the last evaluated
/// expression statement (matches the `Vortex::execute` contract).
///
/// If the script calls `console.log`, the output is captured in the
/// [`Vortex`] instance — to drain it, use [`Vortex::execute_with_console`]
/// instead. This function does **not** return console output.
pub fn vortex_eval(source: &str) -> VortexResult<JsValue> {
    let mut runtime = Vortex::new();
    runtime.init()?;
    runtime.execute(source)
}
