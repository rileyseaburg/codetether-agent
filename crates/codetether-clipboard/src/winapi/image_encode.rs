//! Normalize native clipboard pixels into a PNG data URL attachment.
use codetether_image::ImageAttachment;
use std::io::Cursor;

/// Decode an encoded native clipboard image and normalize it as PNG.
pub(super) fn encode_png_attachment(
    bytes: &[u8],
    format: image::ImageFormat,
) -> Option<ImageAttachment> {
    let decoded = image::ImageReader::with_format(Cursor::new(bytes), format)
        .decode()
        .ok()?;
    let mut png = Vec::new();
    decoded
        .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .ok()?;
    Some(ImageAttachment {
        data_url: codetether_image::encode_data_url("image/png", &png),
        mime_type: Some("image/png".to_string()),
    })
}
