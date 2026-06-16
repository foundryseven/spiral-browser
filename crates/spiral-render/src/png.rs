//! PNG encoding for software-rendered framebuffers.

use png::ColorType;
use thiserror::Error;

use crate::software::SoftwareRenderer;

#[derive(Debug, Error)]
pub enum PngError {
    #[error("png encoder error: {0}")]
    Png(String),
}

/// Encode the renderer's current framebuffer as an RGBA8 PNG byte stream.
pub fn encode_png(renderer: &SoftwareRenderer) -> Result<Vec<u8>, PngError> {
    let (w, h) = (renderer.width(), renderer.height());
    let pixels = renderer.pixels();

    // png crate wants a flat u8 slice; the renderer stores [R, G, B, A] already.
    let mut out = Vec::with_capacity(pixels.len() * 4);
    for px in pixels {
        out.extend_from_slice(px);
    }

    let mut sink: Vec<u8> = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut sink, w, h);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder
            .write_header()
            .map_err(|e| PngError::Png(e.to_string()))?;
        writer
            .write_image_data(&out)
            .map_err(|e| PngError::Png(e.to_string()))?;
        writer.finish().map_err(|e| PngError::Png(e.to_string()))?;
    }
    Ok(sink)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::software::SoftwareRenderer;
    use spiral_paint::{Color, DisplayList, RenderOp};

    #[test]
    fn encodes_valid_png_header() {
        let mut r = SoftwareRenderer::new(2, 2, [10, 20, 30, 255]).unwrap();
        let list = DisplayList {
            ops: vec![RenderOp::FillRect {
                x: 0.0,
                y: 0.0,
                width: 2.0,
                height: 2.0,
                color: Color {
                    r: 200,
                    g: 100,
                    b: 50,
                    a: 1.0,
                },
            }],
        };
        r.draw(&list).unwrap();
        let bytes = encode_png(&r).unwrap();
        // PNG signature: 89 50 4E 47 0D 0A 1A 0A (8 bytes).
        assert_eq!(
            &bytes[0..8],
            &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]
        );
        // First chunk type is IHDR.
        assert_eq!(&bytes[12..16], b"IHDR");
    }
}
