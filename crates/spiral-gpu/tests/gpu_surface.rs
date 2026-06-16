//! Integration tests for the `spiral-gpu` public surface.

use spiral_gpu::GpuDevice;

#[test]
fn test_gpu_surface_wiring() {
    let device = GpuDevice::new();
    assert_eq!(device.name(), "Software Renderer");
    assert!(device.is_available());
}
