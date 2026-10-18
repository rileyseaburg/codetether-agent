//! File picker image.
use std::path::Path;

const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "bmp", "svg"];

/// Whether the extension identifies a supported image preview type.
/// # Arguments
/// * `path` - Filename to inspect without reading it.
/// # Returns
/// True for a recognized extension, ignoring ASCII case.
/// # Examples
/// ```
/// assert!(codetether_file_picker::is_image_file(std::path::Path::new("photo.PNG")));
/// ```
pub fn is_image_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
        .is_some_and(|ext| IMAGE_EXTENSIONS.contains(&ext.as_str()))
}
