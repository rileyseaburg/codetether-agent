//! MIME labels supported by image file attachment.
use std::path::Path;

/// Human-readable list of recognized filename extensions.
pub const SUPPORTED_IMAGE_FORMATS: &str = "png, jpg/jpeg, gif, webp, bmp, svg";

/// Resolve a MIME label from a filename extension, ignoring ASCII case.
///
/// # Arguments
/// * `path` — Filename whose extension is examined.
///
/// # Returns
/// The recognized MIME label, or None for an unsupported extension.
///
/// # Examples
/// ```
/// assert_eq!(codetether_image::guess_mime(std::path::Path::new("PHOTO.PNG")).as_deref(), Some("image/png"));
/// ```
pub fn guess(path: &Path) -> Option<String> {
    match extension(path).as_deref() {
        Some("png") => Some("image/png".to_string()),
        Some("jpg") | Some("jpeg") => Some("image/jpeg".to_string()),
        Some("gif") => Some("image/gif".to_string()),
        Some("webp") => Some("image/webp".to_string()),
        Some("bmp") => Some("image/bmp".to_string()),
        Some("svg") => Some("image/svg+xml".to_string()),
        _ => None,
    }
}

fn extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
}
