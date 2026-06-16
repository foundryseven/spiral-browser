//! Software rasteriser for the Spiral Browser display list.
//!
//! Phase 1 software path. Takes a `DisplayList` (whose `RenderOp`s are nested
//! `Clip`/`Transform` scopes) and produces an RGBA8 framebuffer ready for PNG
//! export or upload to a GPU texture.
//!
//! Vello is wired in the manifest but not used in Phase 1 — it requires a GPU
//! surface and a window, neither of which exist in headless tests.

use thiserror::Error;

use spiral_paint::{Color, DisplayList, RenderOp};

use crate::font;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("width and height must be > 0 (got {width}x{height})")]
    InvalidSize { width: u32, height: u32 },
    #[error("layer stack underflow (PopLayer without PushLayer)")]
    LayerUnderflow,
}

/// 32-bit RGBA8 pixel.
pub type Rgba = [u8; 4];

/// 2D affine transform as a 3x3 matrix stored in row-major order:
/// `[a, b, c, d, e, f]` represents
/// `x' = a*x + c*y + e`, `y' = b*x + d*y + f`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl Transform {
    pub const IDENTITY: Self = Self {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 0.0,
        f: 0.0,
    };

    #[inline]
    pub const fn translate(tx: f32, ty: f32) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: tx,
            f: ty,
        }
    }

    /// Map a point through the transform.
    #[inline]
    pub fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.a * x + self.c * y + self.e,
            self.b * x + self.d * y + self.f,
        )
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<Transform> for [f32; 6] {
    fn from(t: Transform) -> Self {
        [t.a, t.b, t.c, t.d, t.e, t.f]
    }
}

impl From<[f32; 6]> for Transform {
    fn from(m: [f32; 6]) -> Self {
        Self {
            a: m[0],
            b: m[1],
            c: m[2],
            d: m[3],
            e: m[4],
            f: m[5],
        }
    }
}

fn compose(outer: Transform, inner: Transform) -> Transform {
    Transform {
        a: outer.a * inner.a + outer.c * inner.b,
        b: outer.b * inner.a + outer.d * inner.b,
        c: outer.a * inner.c + outer.c * inner.d,
        d: outer.b * inner.c + outer.d * inner.d,
        e: outer.a * inner.e + outer.c * inner.f + outer.e,
        f: outer.b * inner.e + outer.d * inner.f + outer.f,
    }
}

fn color_to_rgba(c: Color) -> Rgba {
    let a = c.a.clamp(0.0, 1.0);
    [c.r, c.g, c.b, (a * 255.0).round().clamp(0.0, 255.0) as u8]
}

fn blend_pixel(dst: Rgba, src: Rgba) -> Rgba {
    let sa = src[3] as f32 / 255.0;
    let inv = 1.0 - sa;
    [
        ((src[0] as f32) * sa + (dst[0] as f32) * inv) as u8,
        ((src[1] as f32) * sa + (dst[1] as f32) * inv) as u8,
        ((src[2] as f32) * sa + (dst[2] as f32) * inv) as u8,
        ((src[3] as f32) * sa + (dst[3] as f32) * inv) as u8,
    ]
}

/// Framebuffer-backed software renderer.
pub struct SoftwareRenderer {
    width: u32,
    height: u32,
    /// Pixels in row-major top-down order, RGBA8.
    pixels: Vec<Rgba>,
    /// Saved layers pushed by `PushLayer` (with their effective opacity).
    layer_stack: Vec<(Vec<Rgba>, f32)>,
}

impl SoftwareRenderer {
    /// Allocate a framebuffer cleared to `bg`.
    pub fn new(width: u32, height: u32, bg: Rgba) -> Result<Self, RenderError> {
        if width == 0 || height == 0 {
            return Err(RenderError::InvalidSize { width, height });
        }
        let pixels = vec![bg; (width as usize) * (height as usize)];
        Ok(Self {
            width,
            height,
            pixels,
            layer_stack: Vec::new(),
        })
    }

    #[inline]
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Borrow the final pixel buffer.
    #[must_use]
    pub fn pixels(&self) -> &[Rgba] {
        &self.pixels
    }

    /// Take ownership of the pixel buffer.
    #[must_use]
    pub fn into_pixels(self) -> Vec<Rgba> {
        self.pixels
    }

    /// Reset to a solid colour, discarding any saved layers.
    pub fn clear(&mut self, bg: Rgba) {
        self.pixels.fill(bg);
        self.layer_stack.clear();
    }

    /// Walk the display list and execute each op in order.
    ///
    /// `Clip`/`Transform`/`PushLayer` introduce scopes; their inner `ops` are
    /// walked with the corresponding state applied. `PopLayer` is ignored
    /// here — scoping is handled by the tree walk.
    pub fn draw(&mut self, list: &DisplayList) -> Result<(), RenderError> {
        self.walk(&list.ops, Transform::IDENTITY, None)
    }

    fn walk(
        &mut self,
        ops: &[RenderOp],
        parent_xform: Transform,
        parent_clip: Option<(i32, i32, i32, i32)>,
    ) -> Result<(), RenderError> {
        for op in ops {
            match op {
                RenderOp::FillRect {
                    x,
                    y,
                    width,
                    height,
                    color,
                } => {
                    self.draw_fill(*x, *y, *width, *height, *color, parent_xform, parent_clip);
                }
                RenderOp::StrokeRect {
                    x,
                    y,
                    width,
                    height,
                    color,
                    stroke_width,
                } => {
                    self.draw_stroke(
                        *x,
                        *y,
                        *width,
                        *height,
                        *color,
                        *stroke_width,
                        parent_xform,
                        parent_clip,
                    );
                }
                RenderOp::DrawText {
                    x,
                    y,
                    text,
                    font_size,
                    color,
                } => {
                    self.draw_text(*x, *y, text, *font_size, *color, parent_xform, parent_clip);
                }
                RenderOp::Clip {
                    x,
                    y,
                    width,
                    height,
                    ops: inner,
                } => {
                    let (cx0, cy0) = parent_xform.apply(*x, *y);
                    let (cx1, cy1) = parent_xform.apply(x + width, y + height);
                    let lo_x = cx0.min(cx1).max(0.0) as i32;
                    let lo_y = cy0.min(cy1).max(0.0) as i32;
                    let hi_x = cx0.max(cx1).min(self.width as f32).ceil() as i32;
                    let hi_y = cy0.max(cy1).min(self.height as f32).ceil() as i32;
                    let new_clip = (lo_x, lo_y, hi_x, hi_y);
                    let combined = match parent_clip {
                        Some(prev) => intersect_clip(prev, new_clip),
                        None => new_clip,
                    };
                    self.walk(inner, parent_xform, Some(combined))?;
                }
                RenderOp::Transform { matrix, ops: inner } => {
                    let inner_xform: Transform = (*matrix).into();
                    let next = compose(parent_xform, inner_xform);
                    self.walk(inner, next, parent_clip)?;
                }
                RenderOp::PushLayer { opacity } => {
                    self.layer_stack.push((self.pixels.clone(), *opacity));
                }
                RenderOp::PopLayer => {
                    let (layer, opacity) =
                        self.layer_stack.pop().ok_or(RenderError::LayerUnderflow)?;
                    self.composite_layer(&layer, opacity);
                }
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_fill(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        xform: Transform,
        clip: Option<(i32, i32, i32, i32)>,
    ) {
        let rgba = color_to_rgba(color);
        let (x0, y0) = xform.apply(x, y);
        let (x1, y1) = xform.apply(x + width, y + height);
        self.fill_rect(x0, y0, x1, y1, rgba, clip);
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_stroke(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        stroke_width: f32,
        xform: Transform,
        clip: Option<(i32, i32, i32, i32)>,
    ) {
        let rgba = color_to_rgba(color);
        let (x0, y0) = xform.apply(x, y);
        let (x1, y1) = xform.apply(x + width, y + height);
        self.stroke_rect(x0, y0, x1, y1, rgba, stroke_width, clip);
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_text(
        &mut self,
        x: f32,
        y: f32,
        text: &str,
        font_size: f32,
        color: Color,
        xform: Transform,
        clip: Option<(i32, i32, i32, i32)>,
    ) {
        if text.is_empty() {
            return;
        }
        let rgba = color_to_rgba(color);
        let (tx, ty) = xform.apply(x, y);
        // font_size scales the 5x7 base glyph uniformly.
        let scale = (font_size / font::GLYPH_HEIGHT as f32).max(0.1);
        let mut cx = tx;
        for raw in text.bytes() {
            let g = font::glyph(raw).unwrap_or_else(font::space_glyph);
            self.blit_glyph(cx, ty, scale, &g, rgba, clip);
            cx += font::GLYPH_ADVANCE as f32 * scale;
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn fill_rect(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: Rgba,
        clip: Option<(i32, i32, i32, i32)>,
    ) {
        let lo_x = x0.min(x1).max(0.0) as i32;
        let lo_y = y0.min(y1).max(0.0) as i32;
        let hi_x = x0.max(x1).min(self.width as f32).ceil() as i32;
        let hi_y = y0.max(y1).min(self.height as f32).ceil() as i32;
        let (lo_x, lo_y, hi_x, hi_y) = apply_clip(lo_x, lo_y, hi_x, hi_y, clip);
        if hi_x <= lo_x || hi_y <= lo_y {
            return;
        }
        for yy in lo_y..hi_y {
            let row_start = (yy as u32 * self.width) as usize;
            for xx in lo_x..hi_x {
                let idx = row_start + xx as usize;
                self.pixels[idx] = blend_pixel(self.pixels[idx], color);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn stroke_rect(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: Rgba,
        stroke_width: f32,
        clip: Option<(i32, i32, i32, i32)>,
    ) {
        let w = stroke_width.max(1.0);
        let half = w * 0.5;
        self.fill_rect(x0 - half, y0 - half, x1 + half, y0 + half, color, clip);
        self.fill_rect(x0 - half, y1 - half, x1 + half, y1 + half, color, clip);
        self.fill_rect(x0 - half, y0, x0 + half, y1, color, clip);
        self.fill_rect(x1 - half, y0, x1 + half, y1, color, clip);
    }

    fn blit_glyph(
        &mut self,
        x: f32,
        y: f32,
        scale: f32,
        glyph: &[u8; font::GLYPH_HEIGHT as usize],
        color: Rgba,
        clip: Option<(i32, i32, i32, i32)>,
    ) {
        let gw = (font::GLYPH_WIDTH as f32 * scale).ceil() as i32;
        let gh = (font::GLYPH_HEIGHT as f32 * scale).ceil() as i32;
        let base_x = x as i32;
        let base_y = y as i32;
        for py in 0..gh {
            let src_y = ((py as f32) / scale) as usize;
            if src_y >= font::GLYPH_HEIGHT as usize {
                continue;
            }
            let row = glyph[src_y];
            for px in 0..gw {
                let src_x = ((px as f32) / scale) as usize;
                if src_x >= font::GLYPH_WIDTH as usize {
                    continue;
                }
                if (row >> src_x) & 1 == 0 {
                    continue;
                }
                let dx = base_x + px;
                let dy = base_y + py;
                if dx < 0 || dy < 0 || dx >= self.width as i32 || dy >= self.height as i32 {
                    continue;
                }
                if let Some((lx, ly, hx, hy)) = clip {
                    if dx < lx || dy < ly || dx >= hx || dy >= hy {
                        continue;
                    }
                }
                let idx = (dy as u32 * self.width + dx as u32) as usize;
                self.pixels[idx] = blend_pixel(self.pixels[idx], color);
            }
        }
    }

    /// Composite the current framebuffer (top) over the saved `before`
    /// snapshot (bottom) at `opacity`, storing the result back into the
    /// framebuffer.
    ///
    /// `before` is what the buffer looked like when `PushLayer` was issued;
    /// `self.pixels` holds the modified state produced inside the layer
    /// scope. After this call the buffer holds `opacity * top + (1 - opacity) * bottom`.
    fn composite_layer(&mut self, before: &[Rgba], opacity: f32) {
        debug_assert_eq!(before.len(), self.pixels.len());
        let alpha = opacity.clamp(0.0, 1.0);
        let inv = 1.0 - alpha;
        for (dst, bottom) in self.pixels.iter_mut().zip(before.iter()) {
            // `dst` is the top of the composite, `bottom` is the destination.
            let top = *dst;
            let out_a = (top[3] as f32) * alpha + (bottom[3] as f32) * inv;
            *dst = [
                ((top[0] as f32) * alpha + (bottom[0] as f32) * inv) as u8,
                ((top[1] as f32) * alpha + (bottom[1] as f32) * inv) as u8,
                ((top[2] as f32) * alpha + (bottom[2] as f32) * inv) as u8,
                out_a as u8,
            ];
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn intersect_clip(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    (a.0.max(b.0), a.1.max(b.1), a.2.min(b.2), a.3.min(b.3))
}

fn apply_clip(
    lo_x: i32,
    lo_y: i32,
    hi_x: i32,
    hi_y: i32,
    clip: Option<(i32, i32, i32, i32)>,
) -> (i32, i32, i32, i32) {
    match clip {
        Some((cx0, cy0, cx1, cy1)) => (lo_x.max(cx0), lo_y.max(cy0), hi_x.min(cx1), hi_y.min(cy1)),
        None => (lo_x, lo_y, hi_x, hi_y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spiral_paint::{Color, DisplayList, RenderOp};

    fn solid_bg() -> Rgba {
        [0, 0, 0, 0]
    }

    fn opaque(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    #[test]
    fn rejects_zero_size() {
        assert!(matches!(
            SoftwareRenderer::new(0, 10, solid_bg()),
            Err(RenderError::InvalidSize { .. })
        ));
        assert!(matches!(
            SoftwareRenderer::new(10, 0, solid_bg()),
            Err(RenderError::InvalidSize { .. })
        ));
    }

    #[test]
    fn clear_resets_buffer() {
        let mut r = SoftwareRenderer::new(4, 4, [0, 0, 0, 255]).unwrap();
        r.fill_rect(0.0, 0.0, 4.0, 4.0, [255, 0, 0, 255], None);
        assert_eq!(r.pixels()[0], [255, 0, 0, 255]);
        r.clear([0, 0, 0, 255]);
        assert_eq!(r.pixels()[0], [0, 0, 0, 255]);
    }

    #[test]
    fn fill_rect_writes_pixels() {
        let mut r = SoftwareRenderer::new(8, 8, [0, 0, 0, 0]).unwrap();
        r.fill_rect(2.0, 2.0, 5.0, 5.0, [10, 20, 30, 255], None);
        assert_eq!(r.pixels()[2 * 8 + 2], [10, 20, 30, 255]);
        assert_eq!(r.pixels()[4 * 8 + 4], [10, 20, 30, 255]);
        assert_eq!(r.pixels()[0], [0, 0, 0, 0]);
    }

    #[test]
    fn draw_handles_nested_clip() {
        let mut r = SoftwareRenderer::new(10, 10, [0, 0, 0, 0]).unwrap();
        let mut list = DisplayList { ops: vec![] };
        list.ops.push(RenderOp::Clip {
            x: 2.0,
            y: 2.0,
            width: 4.0,
            height: 4.0,
            ops: vec![RenderOp::FillRect {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
                color: opaque(255, 255, 255),
            }],
        });
        r.draw(&list).unwrap();
        assert_eq!(r.pixels()[0], [0, 0, 0, 0]);
        assert_eq!(r.pixels()[3 * 10 + 3], [255, 255, 255, 255]);
    }

    #[test]
    fn text_renders_glyphs() {
        let mut r = SoftwareRenderer::new(60, 20, [0, 0, 0, 0]).unwrap();
        let list = DisplayList {
            ops: vec![RenderOp::DrawText {
                x: 0.0,
                y: 0.0,
                text: "A".to_string(),
                font_size: 7.0,
                color: opaque(255, 255, 255),
            }],
        };
        r.draw(&list).unwrap();
        let drawn = r.pixels().iter().filter(|p| p[3] > 0).count();
        assert!(drawn > 0, "expected at least one drawn pixel, got 0");
    }

    #[test]
    fn layer_underflow_is_an_error() {
        let mut r = SoftwareRenderer::new(4, 4, [0, 0, 0, 255]).unwrap();
        let list = DisplayList {
            ops: vec![RenderOp::PopLayer],
        };
        assert!(matches!(r.draw(&list), Err(RenderError::LayerUnderflow)));
    }

    #[test]
    fn transform_translates_origin() {
        let mut r = SoftwareRenderer::new(8, 8, [0, 0, 0, 0]).unwrap();
        let list = DisplayList {
            ops: vec![RenderOp::Transform {
                matrix: Transform::translate(2.0, 3.0).into(),
                ops: vec![RenderOp::FillRect {
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 1.0,
                    color: opaque(255, 0, 0),
                }],
            }],
        };
        r.draw(&list).unwrap();
        assert_eq!(r.pixels()[3 * 8 + 2], [255, 0, 0, 255]);
    }

    #[test]
    fn push_pop_layer_round_trips_buffer() {
        let mut r = SoftwareRenderer::new(2, 2, [10, 20, 30, 255]).unwrap();
        let list = DisplayList {
            ops: vec![
                RenderOp::PushLayer { opacity: 1.0 },
                RenderOp::FillRect {
                    x: 0.0,
                    y: 0.0,
                    width: 2.0,
                    height: 2.0,
                    color: opaque(200, 100, 50),
                },
                RenderOp::PopLayer,
            ],
        };
        r.draw(&list).unwrap();
        // After pop, the layer is composited back at full opacity.
        assert_eq!(r.pixels()[0], [200, 100, 50, 255]);
    }
}
