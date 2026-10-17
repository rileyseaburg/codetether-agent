//! Portable capture checks require no desktop, input, or OS handles.
use codetether_platform::capture::{MAX_PIXELS, bgra_to_png, pixel_bytes};

#[test]
fn allocation_limits_preserve_original_dimensions() {
    assert_eq!(pixel_bytes(3840, 2160).unwrap(), 33_177_600);
    assert_eq!(pixel_bytes(MAX_PIXELS as i64, 1).unwrap(), MAX_PIXELS as usize * 4);
    assert!(pixel_bytes(MAX_PIXELS as i64 + 1, 1).is_err());
    for (width, height) in [(0, 10), (-1, 20), (i64::MAX, 2), (100_000, 100_000)] {
        assert!(pixel_bytes(width, height).is_err());
    }
}

#[test]
fn encoder_normalizes_reserved_alpha_and_bgra_order() {
    let encoded = bgra_to_png(2, 1, vec![5, 10, 20, 0, 30, 40, 50, 100]).unwrap();
    let decoded = image::load_from_memory(&encoded).unwrap().to_rgba8();
    assert_eq!(decoded.dimensions(), (2, 1));
    assert_eq!(decoded.as_raw(), &[20, 10, 5, 255, 50, 40, 30, 255]);
}

#[test]
fn encoder_rejects_bad_buffers_before_encoding() {
    assert!(bgra_to_png(2, 2, vec![0; 4]).is_err());
    assert!(bgra_to_png(1, 1, vec![0; 5]).is_err());
    assert!(bgra_to_png(0, 1, vec![]).is_err());
}
