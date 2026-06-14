//! Spiral Browser — 2D GPU Renderer
//!
//! 2D GPU rendering pipeline for the Spiral Browser.

use spiral_core::Result;
use spiral_paint::{DisplayList, RenderOp};

/// Renderer state.
pub struct Renderer {
    /// Width of the render target.
    width: f32,
    /// Height of the render target.
    height: f32,
}

impl Renderer {
    /// Create a new renderer.
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Render a display list.
    pub fn render(&self, display_list: &DisplayList) -> Result<()> {
        // Phase 1: Software rendering
        // Phase 2: Vello + wgpu GPU rendering
        for op in &display_list.ops {
            self.execute_op(op)?;
        }
        Ok(())
    }

    /// Execute a single render operation.
    fn execute_op(&self, op: &RenderOp) -> Result<()> {
        match op {
            RenderOp::FillRect {
                x,
                y,
                width,
                height,
                color,
            } => {
                // Software fill rect (placeholder)
                log::trace!(
                    "FillRect: ({}, {}) {}x{} rgba({},{},{},{})",
                    x,
                    y,
                    width,
                    height,
                    color.r,
                    color.g,
                    color.b,
                    color.a
                );
                Ok(())
            }
            RenderOp::StrokeRect {
                x,
                y,
                width,
                height,
                color,
                stroke_width,
            } => {
                log::trace!(
                    "StrokeRect: ({}, {}) {}x{} width={} rgba({},{},{},{})",
                    x,
                    y,
                    width,
                    height,
                    stroke_width,
                    color.r,
                    color.g,
                    color.b,
                    color.a
                );
                Ok(())
            }
            RenderOp::DrawText {
                x,
                y,
                text,
                font_size,
                color,
            } => {
                log::trace!(
                    "DrawText: ({}, {}) '{}' size={} rgba({},{},{},{})",
                    x,
                    y,
                    text,
                    font_size,
                    color.r,
                    color.g,
                    color.b,
                    color.a
                );
                Ok(())
            }
            RenderOp::Clip { ops, .. } => {
                for op in ops {
                    self.execute_op(op)?;
                }
                Ok(())
            }
            RenderOp::Transform { ops, .. } => {
                for op in ops {
                    self.execute_op(op)?;
                }
                Ok(())
            }
            RenderOp::PushLayer { opacity } => {
                log::trace!("PushLayer opacity={}", opacity);
                Ok(())
            }
            RenderOp::PopLayer => {
                log::trace!("PopLayer");
                Ok(())
            }
        }
    }

    /// Resize the render target.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spiral_paint::{Color, DisplayList};

    #[test]
    fn test_render_empty() {
        let renderer = Renderer::new(800.0, 600.0);
        let display_list = DisplayList { ops: Vec::new() };
        renderer.render(&display_list).unwrap();
    }

    #[test]
    fn test_render_fill_rect() {
        let renderer = Renderer::new(800.0, 600.0);
        let display_list = DisplayList {
            ops: vec![RenderOp::FillRect {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
                color: Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 1.0,
                },
            }],
        };
        renderer.render(&display_list).unwrap();
    }

    #[test]
    fn test_resize() {
        let mut renderer = Renderer::new(800.0, 600.0);
        renderer.resize(1024.0, 768.0);
        assert_eq!(renderer.width, 1024.0);
        assert_eq!(renderer.height, 768.0);
    }
}
