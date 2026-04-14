use std::path::{Path, PathBuf};

/// Resolve the base directory used for worker-managed clone checkouts.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker_workspace_context::git_clone_base_dir;
///
/// let root = git_clone_base_dir();
/// assert!(!root.as_os_str().is_empty());
/// ```
pub fn git_clone_base_dir() -> PathBuf {
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

/// Normalize a stored workspace path against the active clone root.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker_workspace_context::resolve_workspace_clone_path;
///
/// let path = resolve_workspace_clone_path("ws-1", "");
/// assert!(path.ends_with("ws-1"));
/// ```
pub fn resolve_workspace_clone_path(workspace_id: &str, preferred_path: &str) -> PathBuf {
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
