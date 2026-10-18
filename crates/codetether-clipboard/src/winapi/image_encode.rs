//! Convert BMP bytes into a PNG data URL attachment.
use base64::Engine;
use codetether_image::ImageAttachment;
use std::io::Cursor;

/// Decode a BMP buffer and re-encode it as a PNG data URL attachment.
pub(super) fn encode_png_attachment(bmp: &[u8]) -> Option<ImageAttachment> {
    let decoded = image::load_from_memory_with_format(bmp, image::ImageFormat::Bmp).ok()?;
    let mut png = Vec::new();
    decoded
        .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .ok()?;
    let data = base64::engine::general_purpose::STANDARD.encode(&png);
    Some(ImageAttachment {
        data_url: format!("data:image/png;base64,{data}"),
        mime_type: Some("image/png".to_string()),
    })
}
