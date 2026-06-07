//! Locate VS Code extension manifests near the workspace.

use std::path::{Path, PathBuf};

pub(super) fn package_json_from(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();
    loop {
        let package = current.join("package.json");
        if package.exists() {
            return Some(package);
        }
        if current.join(".git").exists() || !current.pop() {
            return None;
        }
    }
}
