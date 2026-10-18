//! Compact MIME and decoded-size descriptions for image attachments.
use crate::ImageAttachment;
/// Describe an image attachment's MIME type and decoded size when available.
/// # Arguments
/// * `image` - Encoded image attachment.
/// # Returns
/// The existing compact image label used in chat input previews.
/// # Examples
/// ```
/// let image = codetether_image::ImageAttachment { data_url: "invalid".into(), mime_type: None };
/// assert_eq!(codetether_image::image_summary(&image), "[image/* image]");
/// ```
pub fn image_summary(image: &ImageAttachment) -> String {
    let mime = image.mime_type.as_deref().unwrap_or("image/*");
    match crate::decoded_len(&image.data_url) {
        Some(bytes) => format!("[{mime} image, {}]", format_bytes(bytes)),
        None => format!("[{mime} image]"),
    }
}
fn format_bytes(bytes: usize) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.1} MiB", bytes as f64 / 1024.0 / 1024.0)
    } else if bytes >= 1024 {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}
