//! Workspace-scoped data directory resolution.

use std::path::{Path, PathBuf};

/// Environment variable that overrides every derived data directory.
pub const DATA_DIR_ENV: &str = "CODETETHER_DATA_DIR";

/// Resolve the CodeTether data directory for the current process.
///
/// Order: `CODETETHER_DATA_DIR`, then `<workspace>/.codetether-agent` for the
/// current directory's workspace, then the platform data directory.
///
/// # Examples
///
/// ```rust
/// let _dir = codetether_paths::data_dir();
/// ```
pub fn data_dir() -> Option<PathBuf> {
    explicit_data_dir().or_else(|| {
        let cwd = std::env::current_dir().ok();
        cwd.map(|cwd| workspace_data_dir_from(&cwd))
            .or_else(platform_data_dir)
    })
}

/// Resolve the data directory for an explicit workspace path.
pub fn data_dir_for_workspace(workspace: &Path) -> Option<PathBuf> {
    explicit_data_dir().or_else(|| Some(workspace_data_dir_from(workspace)))
}

/// `<workspace root or start>/.codetether-agent`.
pub fn workspace_data_dir_from(start: &Path) -> PathBuf {
    detect_workspace_root(start)
        .unwrap_or_else(|| start.to_path_buf())
        .join(".codetether-agent")
}

/// Find the nearest ancestor containing a `.git` marker.
///
/// Ancestors too broad to scope a workspace (the filesystem root or the
/// user's home directory) are rejected.
pub fn detect_workspace_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|path| !crate::is_unsafe_workspace_root(path) && path.join(".git").exists())
        .map(Path::to_path_buf)
}

fn explicit_data_dir() -> Option<PathBuf> {
    let explicit = std::env::var(DATA_DIR_ENV).ok()?;
    let explicit = explicit.trim();
    (!explicit.is_empty()).then(|| PathBuf::from(explicit))
}

fn platform_data_dir() -> Option<PathBuf> {
    crate::project_dirs().map(|dirs| dirs.data_dir().to_path_buf())
}

#[cfg(test)]
#[path = "data_dir_tests.rs"]
mod tests;
