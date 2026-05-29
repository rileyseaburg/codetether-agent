use std::path::Path;

use crate::tui::app::file_preview::read_file_preview_lines;

use super::image::is_image_file;
use super::types::FilePreview;

pub const PREVIEW_MAX_BYTES: usize = 64 * 1024;
pub const PREVIEW_MAX_LINES: usize = 1000;

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
            lines: binary_lines(path, lines, binary),
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

fn binary_lines(path: &Path, lines: Vec<String>, binary: bool) -> Vec<String> {
    if binary {
        vec![format!("Binary file: {}", path.display())]
    } else {
        lines
    }
}
