//! Read supported image files into portable data-URL attachments.
use std::path::Path;

use crate::ImageAttachment;

/// Read a regular file and encode it with its extension-derived MIME label.
///
/// # Arguments
/// * `path` — Image file to read.
///
/// # Returns
/// The encoded image attachment.
///
/// # Errors
/// Returns a message when the path is missing, is not a file, cannot be read,
/// or has an unsupported extension.
///
/// # Examples
/// ```no_run
/// let image = codetether_image::attachment_from_file(std::path::Path::new("photo.png")).unwrap();
/// assert_eq!(image.mime_type.as_deref(), Some("image/png"));
/// ```
pub fn attachment_from_file(path: &Path) -> Result<ImageAttachment, String> {
    ensure_regular_file(path)?;
    let bytes =
        std::fs::read(path).map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
    let mime_type = crate::file_mime::guess(path).ok_or_else(|| unsupported(path))?;
    tracing::info!(path = %path.display(), mime = %mime_type, size_bytes = bytes.len(), "Attached image file");
    Ok(ImageAttachment {
        data_url: crate::encoding::encode(&mime_type, &bytes),
        mime_type: Some(mime_type),
    })
}

fn ensure_regular_file(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }
    if !path.is_file() {
        return Err(format!("Not a file: {}", path.display()));
    }
    Ok(())
}

fn unsupported(path: &Path) -> String {
    format!(
        "Unsupported image format: {}. Supported: {}",
        path.display(),
        crate::file_mime::SUPPORTED_IMAGE_FORMATS
    )
}
