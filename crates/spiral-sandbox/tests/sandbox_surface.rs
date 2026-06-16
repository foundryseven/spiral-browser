//! Integration tests for the `spiral-sandbox` public surface.

use spiral_sandbox::{Sandbox, SandboxConfig};

#[test]
fn test_sandbox_surface_wiring() {
    let _config = SandboxConfig {
        restrict_filesystem: true,
        restrict_network: true,
        restrict_process: true,
    };
    let sandbox = Sandbox::new();
    assert!(!sandbox.is_active());
}
