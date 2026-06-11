//! File collection for native fallback grep.

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub(super) fn collect(path: &str) -> Vec<PathBuf> {
    let path = Path::new(path);
    if path.is_file() {
        return vec![path.to_path_buf()];
    }
    WalkDir::new(path)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_path_buf())
        .collect()
}
