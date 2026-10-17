//! mime for image loading.
use super::*;
impl ImageTool {
    /// Detect MIME type from file extension
    pub(super) fn detect_mime_type(path: &str) -> &'static str {
        let lower = path.to_lowercase();
        if lower.ends_with(".png") {
            "image/png"
        } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
            "image/jpeg"
        } else if lower.ends_with(".gif") {
            "image/gif"
        } else if lower.ends_with(".webp") {
            "image/webp"
        } else if lower.ends_with(".svg") {
            "image/svg+xml"
        } else if lower.ends_with(".bmp") {
            "image/bmp"
        } else if lower.ends_with(".tiff") || lower.ends_with(".tif") {
            "image/tiff"
        } else {
            "image/jpeg" // Default fallback
        }
    }
}
