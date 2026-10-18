//! Managed worktrees share their owning workspace's canonical session database.
use std::path::{Path, PathBuf};
/// Resolve the data owner, honoring explicit configuration before worktrees.
/// # Returns
/// The configured data directory or the managed worktree owner's directory.
/// # Examples
/// ```rust
/// let directory: Option<std::path::PathBuf> = codetether_session_location::data_dir();
/// # let _ = directory;
/// ```
pub fn data_dir() -> Option<PathBuf> {
    if std::env::var("CODETETHER_DATA_DIR").is_ok_and(|v| !v.trim().is_empty()) {
        return codetether_config::Config::data_dir();
    }
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(root) = owner(&cwd) {
            return Some(root.join(".codetether-agent"));
        }
    }
    codetether_config::Config::data_dir()
}
fn owner(path: &Path) -> Option<&Path> {
    path.ancestors()
        .find(|p| p.file_name().is_some_and(|n| n == ".codetether-worktrees"))?
        .parent()
}
