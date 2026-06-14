//! Spiral Browser — Image Decoder
//!
//! Image decoding for the Spiral Browser.

use spiral_core::{Error, Result};

/// Supported image formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
    Avif,
}

/// Decoded image.
pub struct DecodedImage {
    /// Image width in pixels.
    pub width: u32,
    /// Image height in pixels.
    pub height: u32,
    /// Image data (RGBA).
    pub data: Vec<u8>,
    /// Image format.
    pub format: ImageFormat,
}

/// Image decoder.
pub struct ImageDecoder;

impl ImageDecoder {
    /// Create a new image decoder.
    pub fn new() -> Self {
        Self
    }

    /// Detect image format from bytes.
    pub fn detect_format(&self, data: &[u8]) -> Option<ImageFormat> {
        if data.len() < 4 {
            return None;
        }

        // PNG signature
        if data[0] == 0x89 && data[1] == b'P' && data[2] == b'N' && data[3] == b'G' {
            return Some(ImageFormat::Png);
        }

        // JPEG signature
        if data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF {
            return Some(ImageFormat::Jpeg);
        }

        // WebP signature
        if data[0] == b'R' && data[1] == b'I' && data[2] == b'F' && data[3] == b'F' {
            return Some(ImageFormat::WebP);
        }

        // AVIF signature (ftyp box)
        if data[4] == b'f' && data[5] == b't' && data[6] == b'y' && data[7] == b'p' {
            return Some(ImageFormat::Avif);
        }

        None
    }

    /// Decode image data.
    pub fn decode(&self, data: &[u8]) -> Result<DecodedImage> {
        let format = self
            .detect_format(data)
            .ok_or_else(|| Error::Parse("Unsupported image format".to_string()))?;

        // Phase 1: Placeholder decoding
        // Phase 2: Per-format decoders (png, zune-jpeg, webp, ravif)
        match format {
            ImageFormat::Png => {
                // Phase 2: png crate
                Ok(DecodedImage {
                    width: 1,
                    height: 1,
                    data: vec![255, 255, 255, 255],
                    format,
                })
            }
            ImageFormat::Jpeg => {
                // Phase 2: zune-jpeg crate
                Ok(DecodedImage {
                    width: 1,
                    height: 1,
                    data: vec![255, 255, 255, 255],
                    format,
                })
            }
            ImageFormat::WebP => {
                // Phase 2: webp crate
                Ok(DecodedImage {
                    width: 1,
                    height: 1,
                    data: vec![255, 255, 255, 255],
                    format,
                })
            }
            ImageFormat::Avif => {
                // Phase 2: ravif crate
                Ok(DecodedImage {
                    width: 1,
                    height: 1,
                    data: vec![255, 255, 255, 255],
                    format,
                })
            }
        }
    }
}

impl Default for ImageDecoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_png() {
        let decoder = ImageDecoder::new();
        let data = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(decoder.detect_format(&data), Some(ImageFormat::Png));
    }

    #[test]
    fn test_detect_jpeg() {
        let decoder = ImageDecoder::new();
        let data = [0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(decoder.detect_format(&data), Some(ImageFormat::Jpeg));
    }

    #[test]
    fn test_detect_webp() {
        let decoder = ImageDecoder::new();
        let data = [b'R', b'I', b'F', b'F', 0x00, 0x00, 0x00, 0x00];
        assert_eq!(decoder.detect_format(&data), Some(ImageFormat::WebP));
    }

    #[test]
    fn test_decode_png() {
        let decoder = ImageDecoder::new();
        let data = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        let image = decoder.decode(&data).unwrap();
        assert_eq!(image.format, ImageFormat::Png);
    }
}
