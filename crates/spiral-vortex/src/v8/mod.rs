//! V8 Oracle — CI compliance testing only.
//!
//! This module is gated behind `--features v8`. It wraps Google's V8
//! via `rusty_v8` to provide a known-good reference implementation
//! for the test harness. The CI runs the same JS snippets through
//! both Vortex and this V8 wrapper and compares outputs.
//!
//! **This is NOT the production engine.** It exists solely as a
//! compliance oracle.

#[cfg(feature = "v8")]
use rusty_v8 as v8;

/// A thin V8 isolate for running JS snippets in CI.
#[cfg(feature = "v8")]
pub struct V8Oracle {
    isolate: v8::OwnedIsolate,
}

#[cfg(feature = "v8")]
impl V8Oracle {
    /// Create a new V8 oracle (initializes V8 if not already done).
    pub fn new() -> Self {
        // Initialize V8 exactly once (process-wide).
        use std::sync::Once;
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });

        let params = v8::CreateParams::default();
        let isolate = v8::Isolate::new(params);
        Self { isolate }
    }

    /// Execute a JS snippet and return the string result.
    pub fn execute(&mut self, source: &str) -> Result<String, String> {
        let scope = &mut v8::HandleScope::new(&mut self.isolate);
        let context = v8::Context::new(scope, Default::default());
        let scope = &mut v8::ContextScope::new(scope, context);

        let code = v8::String::new(scope, source).ok_or("failed to create V8 string")?;
        let script = v8::Script::compile(scope, code, None).ok_or("V8 compilation failed")?;
        let result = script.run(scope).ok_or("V8 execution failed")?;
        let str_result = result.to_string(scope).ok_or("V8 ToString failed")?;
        Ok(str_result.to_rust_string_lossy(scope))
    }
}
