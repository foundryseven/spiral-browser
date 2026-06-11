//! Spiral Browser — JavaScript Engine
//!
//! JavaScript engine integration for the Spiral Browser.

use spiral_core::{Error, Result};

/// JavaScript engine wrapper.
pub struct JsEngine {
    /// Engine is initialized.
    initialized: bool,
}

impl JsEngine {
    /// Create a new JavaScript engine.
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }

    /// Initialize the engine.
    pub fn init(&mut self) -> Result<()> {
        // Phase 1: Basic engine setup
        // Phase 2: Boa engine integration
        self.initialized = true;
        log::info!("JavaScript engine initialized");
        Ok(())
    }

    /// Execute JavaScript code.
    pub fn execute(&self, code: &str) -> Result<String> {
        if !self.initialized {
            return Err(Error::JavaScript("Engine not initialized".to_string()));
        }

        // Phase 1: Basic execution (placeholder)
        // Phase 2: Boa engine execution
        log::trace!("Executing JavaScript: {}", code);
        Ok(String::new())
    }

    /// Check if engine is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for JsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_engine() {
        let engine = JsEngine::new();
        assert!(!engine.is_initialized());
    }

    #[test]
    fn test_init_engine() {
        let mut engine = JsEngine::new();
        engine.init().unwrap();
        assert!(engine.is_initialized());
    }

    #[test]
    fn test_execute_before_init() {
        let engine = JsEngine::new();
        assert!(engine.execute("console.log('hello')").is_err());
    }

    #[test]
    fn test_execute_after_init() {
        let mut engine = JsEngine::new();
        engine.init().unwrap();
        engine.execute("console.log('hello')").unwrap();
    }
}
