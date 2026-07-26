//! Guard against unsafe workspace roots.
//!
//! A workspace root that is the user's home directory or a filesystem root
//! makes every derived store (sessions, indexes) span the entire machine. A
//! single knowledge-graph index built from `/home/riley` reached 1.9 GB across
//! 463,795 files, so these locations are rejected as workspace roots.

use std::path::Path;

/// Returns `true` when `path` is too broad to serve as a workspace root.
///
/// Rejects the filesystem root, any path with no parent, and the current
/// user's home directory.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::config::is_unsafe_workspace_root;
/// use std::path::Path;
///
/// assert!(is_unsafe_workspace_root(Path::new("/")));
/// assert!(!is_unsafe_workspace_root(Path::new("/home/user/project")));
/// ```
pub fn is_unsafe_workspace_root(path: &Path) -> bool {
    if path.parent().is_none() {
        return true;
    }
    if path == Path::new("/") {
        return true;
    }
    home_dir().is_some_and(|home| path == home)
}

fn home_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("HOME")
        .filter(|value| !value.is_empty())
        .map(std::path::PathBuf::from)
}
