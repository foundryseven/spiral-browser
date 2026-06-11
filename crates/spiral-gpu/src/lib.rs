//! Spiral Browser — GPU Abstraction
//!
//! GPU abstraction layer for the Spiral Browser.

use spiral_core::{Error, Result};

/// GPU device wrapper.
pub struct GpuDevice {
    /// Device name.
    name: String,
    /// Device is available.
    available: bool,
}

impl GpuDevice {
    /// Create a new GPU device wrapper.
    pub fn new() -> Self {
        Self {
            name: "Software Renderer".to_string(),
            available: true,
        }
    }

    /// Get device name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if device is available.
    pub fn is_available(&self) -> bool {
        self.available
    }
}

impl Default for GpuDevice {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_device() {
        let device = GpuDevice::new();
        assert!(device.is_available());
        assert_eq!(device.name(), "Software Renderer");
    }
}
