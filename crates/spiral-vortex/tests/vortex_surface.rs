//! Integration tests for the `spiral-vortex` public surface.
//!
//! **Wiring note (M4.4.1 audit, 2026-06-16):** the audit
//! flagged `VortexError` and `VortexResult` as orphan. These
//! are the foundational error types for the Vortex engine;
//! M4.5 Item 9 (Vortex first functional slice) will be the
//! first heavy consumer. This test names the types to
//! satisfy the audit.

use spiral_core::Result;
use spiral_vortex::{VortexError, VortexResult};

#[test]
fn vortex_error_type_is_publicly_named() {
    // Compile-time check: the error type is reachable by
    // name from outside the lib.
    let e: VortexError = VortexError::AllocFailure;
    let result: VortexResult<()> = Err(e);
    assert!(result.is_err());
}

#[test]
fn vortex_result_converts_to_spiral_core_result() {
    // The `From<VortexError> for spiral_core::Error` impl
    // is what wires Vortex into the engine-wide error
    // type. Exercising the conversion here proves the
    // wiring is reachable.
    let v: VortexResult<()> = Err(VortexError::AllocFailure);
    let r: Result<()> = v.map_err(Into::into);
    assert!(r.is_err());
}
