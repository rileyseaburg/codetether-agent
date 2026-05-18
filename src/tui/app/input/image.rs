//! Image attachment helpers for the TUI chat.

use std::path::Path;

use crate::session::ImageAttachment;


pub(crate) fn attach_image_file(path: &Path) -> Result<ImageAttachment, String> {
    super::image_file::attach(path)
}

pub(crate) fn image_summary(image: &ImageAttachment) -> String {
    let mime = image.mime_type.as_deref().unwrap_or("image/*");
    match super::image_data_url::decoded_len(&image.data_url) {
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
