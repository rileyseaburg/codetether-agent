//! Clone location helpers for worker-managed repositories.
//!
//! This module keeps clone-root selection and workspace-path normalization out
//! of the main worker task loop.
//!
//! # Examples
//!
//! ```ignore
//! let path = resolve_workspace_clone_path("ws-1", "");
//! assert!(path.ends_with("ws-1"));
//! ```

use std::path::{Path, PathBuf};

/// Resolves the base directory used for cloned workspaces.
///
/// The worker prefers `GIT_CLONE_BASE`, then the legacy system directory, and
/// finally a user-scoped path under `$HOME`.
///
/// # Examples
///
/// ```ignore
/// let root = git_clone_base_dir();
/// assert!(root.is_absolute());
/// ```
pub(super) fn git_clone_base_dir() -> PathBuf {
    if let Ok(path) = std::env::var("GIT_CLONE_BASE") {
        return PathBuf::from(path);
    }
    let system_root = PathBuf::from("/var/lib/codetether/repos");
    if system_root.exists() {
        return system_root;
    }
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join(".local/share/codetether/repos"))
        .unwrap_or(system_root)
}

/// Normalizes a workspace clone path against the configured clone root.
///
/// Legacy workspace records may still point at `/var/lib/codetether/repos`.
/// When that happens, the worker remaps them to the active clone root.
///
/// # Examples
///
/// ```ignore
/// let path = resolve_workspace_clone_path("ws-1", "/var/lib/codetether/repos/ws-1");
/// assert!(path.ends_with("ws-1"));
/// ```
pub(super) fn resolve_workspace_clone_path(workspace_id: &str, preferred_path: &str) -> PathBuf {
    let preferred = preferred_path.trim();
    if preferred.is_empty() {
        return git_clone_base_dir().join(workspace_id);
    }
    let preferred_path = PathBuf::from(preferred);
    let legacy_root = Path::new("/var/lib/codetether/repos");
    let configured_root = git_clone_base_dir();
    if preferred_path.starts_with(legacy_root)
        && preferred_path.file_name().and_then(|name| name.to_str()) == Some(workspace_id)
        && configured_root != legacy_root
    {
        return configured_root.join(workspace_id);
    }
    preferred_path
}
