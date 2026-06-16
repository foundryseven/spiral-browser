//! Integration tests for the `spiral-imagedecoder` public surface.

use spiral_imagedecoder::{DecodedImage, ImageDecoder, ImageFormat};

#[test]
fn test_imagedecoder_surface_wiring() {
    let decoder = ImageDecoder::new();
    let format = ImageFormat::Png;
    let img = DecodedImage {
        width: 1,
        height: 1,
        data: vec![0, 0, 0, 0],
        format,
    };
    assert_eq!(img.width, 1);
    assert_eq!(img.height, 1);
    assert_eq!(img.format, ImageFormat::Png);
    let detected = decoder.detect_format(&[0x89, b'P', b'N', b'G']);
    assert_eq!(detected, Some(ImageFormat::Png));
}
