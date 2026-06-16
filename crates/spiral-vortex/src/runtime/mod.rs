//! Vortex runtime — the main entry point for executing JavaScript.
//!
//! A `Vortex` instance owns:
//! - An `Interpreter` (Phase 1) or `Vm` (Phase 2+).
//! - A `VortexHeap` (origin-tagged GC-managed object store).
//! - An event loop (microtask queue, macrotask queue).
//! - A console output buffer.
//!
//! Callers interact with Vortex through this struct; the internals
//! (interpreter vs VM, GC strategy) are implementation details.

use crate::error::VortexResult;
use crate::gc::VortexHeap;
use crate::lexer;
use crate::parser;
use crate::value::JsValue;
use crate::vm::interpreter::Interpreter;

/// The Vortex JavaScript runtime.
pub struct Vortex {
    /// The tree-walking interpreter (Phase 1).
    /// Replaced by a bytecode VM in Phase 2.
    interpreter: Interpreter,
    /// GC heap (Phase 1: per-origin mark-sweep).
    heap: VortexHeap,
    /// Whether the runtime has been initialized.
    initialized: bool,
}

impl Vortex {
    /// Create a new Vortex runtime.
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            heap: VortexHeap::new(),
            initialized: false,
        }
    }

    /// Initialize the runtime (register builtins, etc.).
    pub fn init(&mut self) -> VortexResult<()> {
        // Phase 2: register global builtins (Object, Array, Math, JSON, etc.)
        self.initialized = true;
        log::info!("Vortex runtime initialized");
        Ok(())
    }

    /// Execute a JavaScript source string and return the result.
    pub fn execute(&mut self, source: &str) -> VortexResult<JsValue> {
        if !self.initialized {
            self.init()?;
        }
        let tokens = lexer::lex(source);
        let program = parser::parse(&tokens)?;
        self.interpreter.run(&program)
    }

    /// Execute a JavaScript source string and return console output.
    pub fn execute_with_console(&mut self, source: &str) -> VortexResult<Vec<String>> {
        self.execute(source)?;
        Ok(self.interpreter.console_out.drain(..).collect())
    }

    /// Get the total number of live GC-managed objects across all origins.
    pub fn gc_live_count(&self) -> usize {
        self.heap.total_live_count()
    }

    /// Borrow the heap for read-only operations.
    pub fn heap(&self) -> &VortexHeap {
        &self.heap
    }

    /// Borrow the heap mutably.
    pub fn heap_mut(&mut self) -> &mut VortexHeap {
        &mut self.heap
    }
}

impl Default for Vortex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_basic() {
        let mut vortex = Vortex::new();
        let result = vortex.execute("1 + 2").unwrap();
        assert!(matches!(result, JsValue::Number(n) if n == 3.0));
    }

    #[test]
    fn test_execute_console() {
        let mut vortex = Vortex::new();
        vortex.init().unwrap();
        let output = vortex
            .execute_with_console(r#"console.log("hello")"#)
            .unwrap();
        assert_eq!(output, vec!["hello"]);
    }

    #[test]
    fn test_execute_var() {
        let mut vortex = Vortex::new();
        let result = vortex.execute("var x = 10; x").unwrap();
        assert!(matches!(result, JsValue::Number(n) if n == 10.0));
    }
}
