//! Image attachment helpers for the TUI chat.
//!
//! Reads an image file from disk, base64-encodes it, and
//! wraps it as an [`ImageAttachment`] ready for chat
//! submission.
//!
//! # Examples
//!
//! ```ignore
//! let img = attach_image_file(Path::new("photo.png"))?;
//! ```

use std::path::Path;

use base64::Engine;

use crate::session::ImageAttachment;

/// Read an image from disk and encode it as a base64 data URL.
///
/// Validates the path exists, guesses the MIME type from the
/// file extension, and returns an [`ImageAttachment`] ready
/// to send with a chat message.
///
/// # Examples
///
/// ```ignore
/// let attachment = attach_image_file(Path::new("photo.png"))?;
/// ```
pub(crate) fn attach_image_file(path: &Path) -> Result<ImageAttachment, String> {
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }
    if !path.is_file() {
        return Err(format!("Not a file: {}", path.display()));
    }

    let bytes =
        std::fs::read(path).map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
    let mime_type = guess_image_mime(path).ok_or_else(|| {
        format!(
            "Unsupported image format: {}. Supported: png, jpg/jpeg, gif, webp, bmp, svg",
            path.display()
        )
    })?;
    let base64_data = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let size_kb = bytes.len() as f64 / 1024.0;
    tracing::info!(
        path = %path.display(),
        mime = %mime_type,
        size_kb = %format_args!("{size_kb:.1}"),
        "Attached image file"
    );
    Ok(ImageAttachment {
        data_url: format!("data:{mime_type};base64,{base64_data}"),
        mime_type: Some(mime_type),
    })
}

/// Guess the MIME type for common image file extensions.
fn guess_image_mime(path: &Path) -> Option<String> {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") => Some("image/png".to_string()),
        Some("jpg") | Some("jpeg") => Some("image/jpeg".to_string()),
        Some("gif") => Some("image/gif".to_string()),
        Some("webp") => Some("image/webp".to_string()),
        Some("bmp") => Some("image/bmp".to_string()),
        Some("svg") => Some("image/svg+xml".to_string()),
        _ => None,
    }
}
