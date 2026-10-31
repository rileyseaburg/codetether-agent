//! Native renderer output must be encoded, not just labeled, as PNG.
use super::ScreenshotData;

#[test]
fn ppm_capture_becomes_png_with_original_pixels() {
    let shot = ScreenshotData::from_ppm(b"P6\n2 1\n255\n\xff\x00\x00\x00\xff\x00").unwrap();
    assert_eq!(
        image::guess_format(&shot.bytes).unwrap(),
        image::ImageFormat::Png
    );
    let decoded = image::load_from_memory(&shot.bytes).unwrap().to_rgb8();
    assert_eq!(decoded.dimensions(), (2, 1));
    assert_eq!(decoded.into_raw(), [255, 0, 0, 0, 255, 0]);
}

#[test]
fn malformed_capture_is_not_returned_as_an_image() {
    for bytes in [b"".as_slice(), b"not an image", b"P6\n2 1\n255\n\xff"] {
        assert!(ScreenshotData::from_ppm(bytes).is_err());
    }
}
