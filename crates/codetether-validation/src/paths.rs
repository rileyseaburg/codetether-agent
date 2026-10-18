//! Workspace-relative normalization and diagnostic display paths.
use std::path::{Path, PathBuf};

pub(super) fn normalize_workspace_path(workspace_dir: &Path, raw_path: &str) -> PathBuf {
    let path = PathBuf::from(raw_path);
    if path.is_absolute() {
        path
    } else {
        workspace_dir.join(path)
    }
}

pub(super) fn relative_display_path(workspace_dir: &Path, path: &Path) -> String {
    path.strip_prefix(workspace_dir)
        .unwrap_or(path)
        .display()
        .to_string()
}
