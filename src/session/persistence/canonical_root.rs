//! Managed worktrees share their owning workspace's canonical session database.
use std::path::{Path, PathBuf};
pub(super) fn data_dir() -> Option<PathBuf> {
    if std::env::var("CODETETHER_DATA_DIR").is_ok_and(|v| !v.trim().is_empty()) {
        return crate::config::Config::data_dir();
    }
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(root) = owner(&cwd) {
            return Some(root.join(".codetether-agent"));
        }
    }
    crate::config::Config::data_dir()
}
fn owner(path: &Path) -> Option<&Path> {
    path.ancestors()
        .find(|p| p.file_name().is_some_and(|n| n == ".codetether-worktrees"))?
        .parent()
}
