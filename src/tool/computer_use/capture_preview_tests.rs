//! Bounded preview and original-coordinate regression tests (no desktop access).

use super::*;

fn png(width: u32, height: u32) -> Vec<u8> {
    let image = image::RgbImage::from_pixel(width, height, image::Rgb([20, 40, 60]));
    let mut output = Cursor::new(Vec::new());
    image.write_to(&mut output, image::ImageFormat::Png).unwrap();
    output.into_inner()
}

#[test]
fn large_capture_uses_a_bounded_preview_without_changing_original_geometry() {
    let bytes = png(4096, 64);
    let preview = prepare(&bytes, 4096, 64).unwrap();
    assert_eq!(preview.mapping["original_width"], 4096);
    assert_eq!(preview.mapping["width"], 2048);
    assert_eq!(preview.mapping["image_to_original_scale_x"], 2.0);
    assert_eq!(preview.mapping["image_to_original_scale_y"], 2.0);
    assert!(preview.mapping["encoded_bytes"].as_u64().unwrap() <= MAX_PREVIEW_BYTES as u64);
    assert_eq!(preview.attachment["mime_type"], "image/jpeg");
}

#[test]
fn small_png_is_retained_as_an_image() {
    let preview = prepare(&png(100, 50), 100, 50).unwrap();
    assert_eq!(preview.mapping["image_to_original_scale_x"], 1.0);
    assert_eq!(preview.attachment["mime_type"], "image/png");
    assert!(prepare(&[], 1, 1).is_err());
    assert!(prepare(&png(1, 1), u32::MAX, u32::MAX).is_err());
}

#[cfg(not(windows))]
#[path = "../../platform/windows/computer_use/encode.rs"]
mod native_encode;

#[test]
#[cfg(not(windows))]
fn gdi_reserved_alpha_is_opaque_and_bad_buffers_are_rejected() {
    let encoded = native_encode::bgra_to_png(1, 1, vec![5, 10, 20, 0]).unwrap();
    assert_eq!(image::load_from_memory(&encoded).unwrap().to_rgba8().as_raw(), &[20, 10, 5, 255]);
    assert!(native_encode::bgra_to_png(2, 2, vec![0; 4]).is_err());
}