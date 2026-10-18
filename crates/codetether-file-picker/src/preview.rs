//! File picker preview.
use std::path::Path;

use crate::read_file_preview_lines;

use super::image::is_image_file;
use super::types::FilePreview;

pub const PREVIEW_MAX_BYTES: usize = 64 * 1024;
pub const PREVIEW_MAX_LINES: usize = 1000;

/// Load a bounded preview or directory/image placeholder.
/// # Arguments
/// * `path` - File or directory to preview.
/// # Returns
/// Preview metadata; read errors are represented in its text.
/// # Examples
/// ```no_run
/// let preview = codetether_file_picker::load_preview(std::path::Path::new("note.txt"));
/// ```
pub fn load_preview(path: &Path) -> FilePreview {
    if path.is_dir() {
        return FilePreview {
            path: path.to_path_buf(),
            lines: vec![format!("Directory: {}", path.display())],
            truncated: false,
            binary: false,
        };
    }
    if is_image_file(path) {
        return FilePreview {
            path: path.to_path_buf(),
            lines: vec![format!("Image file: {}", path.display())],
            truncated: false,
            binary: false,
        };
    }
    match read_file_preview_lines(path, PREVIEW_MAX_BYTES, PREVIEW_MAX_LINES) {
        Ok((lines, truncated, binary)) => FilePreview {
            path: path.to_path_buf(),
            lines: super::binary_lines::binary_lines(path, lines, binary),
            truncated,
            binary,
        },
        Err(err) => FilePreview {
            path: path.to_path_buf(),
            lines: vec![format!("Failed to read file: {err}")],
            truncated: false,
            binary: false,
        },
    }
}
