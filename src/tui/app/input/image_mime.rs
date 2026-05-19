use std::path::Path;

pub(crate) const SUPPORTED_IMAGE_FORMATS: &str = "png, jpg/jpeg, gif, webp, bmp, svg";

pub(crate) fn guess(path: &Path) -> Option<String> {
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
