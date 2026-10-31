//! Mocked native clipboard formats exercise the production decoder and format order.
use super::{
    image_fixtures as fixtures,
    image_formats::{Format, capture},
};

#[test]
fn png_only_clipboard_preserves_pixels_and_transparency() {
    let result = capture(|format| {
        assert_eq!(format, Format::Png);
        Some(fixtures::png())
    })
    .unwrap();
    let pixels = fixtures::decoded(result).to_rgba8();
    assert_eq!(pixels.dimensions(), (2, 1));
    assert_eq!(pixels.into_raw(), [16, 32, 48, 128, 64, 80, 96, 255]);
}

#[test]
fn modern_and_legacy_native_bitmaps_become_png() {
    for (available, bytes) in [
        (Format::DibV5, fixtures::dib_v5()),
        (Format::Dib, fixtures::dib()),
    ] {
        let result = capture(|format| (format == available).then(|| bytes.clone())).unwrap();
        let pixels = fixtures::decoded(result).to_rgb8();
        assert_eq!(pixels.dimensions(), (2, 1));
        assert_eq!(pixels.into_raw(), [16, 32, 48, 64, 80, 96]);
    }
}

#[test]
fn corrupt_png_uses_modern_bitmap_instead() {
    let mut attempted = Vec::new();
    let result = capture(|format| {
        attempted.push(format);
        match format {
            Format::Png => Some(b"not a PNG".to_vec()),
            Format::DibV5 => Some(fixtures::dib_v5()),
            Format::Dib => panic!("valid DIBV5 should take precedence"),
        }
    });
    assert!(result.is_some());
    assert_eq!(attempted, [Format::Png, Format::DibV5]);
}

#[test]
fn absent_or_malformed_formats_do_not_create_attachments() {
    assert!(capture(|_| None).is_none());
    assert!(capture(|_| Some(vec![0; 4])).is_none());
}
