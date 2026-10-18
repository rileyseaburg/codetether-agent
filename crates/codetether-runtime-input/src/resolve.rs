//! Workspace-relative tool path handling.
use std::path::{Path, PathBuf};

/// Resolve a raw path string against a workspace root.
///
/// # Examples
///
/// ```rust
/// use codetether_runtime_input::resolve_workspace_path;
/// use std::path::{Path, PathBuf};
///
/// let path = resolve_workspace_path(Path::new("/workspace"), "src/lib.rs");
/// assert_eq!(path, PathBuf::from("/workspace/src/lib.rs"));
/// ```
/// # Arguments
/// Receives a workspace root and raw path string.
/// # Returns
/// An absolute or workspace-relative path, preserving URI and home-relative forms.
pub fn resolve_workspace_path(workspace_dir: &Path, raw: &str) -> PathBuf {
    if raw.is_empty() || raw.contains("://") || raw.starts_with("data:") || raw.starts_with('~') {
        return PathBuf::from(raw);
    }
    let candidate = PathBuf::from(raw);
    if candidate.is_absolute() {
        candidate
    } else {
        workspace_dir.join(candidate)
    }
}
