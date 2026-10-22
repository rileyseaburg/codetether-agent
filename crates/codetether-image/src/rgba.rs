//! Convert raw RGBA pixels to a PNG attachment.
use crate::ImageAttachment;
use base64::Engine;
use std::io::Cursor;
/// Encode raw RGBA pixels as a PNG data-URL attachment.
///
/// # Arguments
/// * `width` - Pixel width, converted to the image library's `u32` size.
/// * `height` - Pixel height, converted to the image library's `u32` size.
/// * `bytes` - RGBA pixel buffer.
///
/// # Returns
/// An attachment, or `None` if the dimensions/buffer or PNG encoding fail.
///
/// # Examples
/// ```
/// let image = codetether_image::attachment_from_rgba(1, 1, vec![0, 0, 0, 255]).unwrap();
/// assert_eq!(image.mime_type.as_deref(), Some("image/png"));
/// ```
pub fn attachment_from_rgba(
    width: usize,
    height: usize,
    bytes: Vec<u8>,
) -> Option<ImageAttachment> {
    let buf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::from_raw(width as u32, height as u32, bytes)?;
    let mut png = Vec::new();
    buf.write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .ok()?;
    let data = base64::engine::general_purpose::STANDARD.encode(&png);
    Some(ImageAttachment {
        data_url: format!("data:image/png;base64,{data}"),
        mime_type: Some("image/png".to_string()),
    })
}
