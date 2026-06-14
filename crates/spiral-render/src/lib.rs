//! Spiral Browser — 2D GPU Renderer
//!
//! Phase 1 ships a software rasteriser that consumes a `DisplayList` and
//! produces an RGBA8 framebuffer (PNG-encodable). Phase 2 will add a Vello +
//! wgpu path that targets a real GPU surface; the `DisplayList` / `RenderOp`
//! API is the contract both paths share.

pub mod font;
pub mod png;
pub mod software;

pub use png::{encode_png, PngError};
pub use software::{RenderError, Rgba, SoftwareRenderer, Transform};
