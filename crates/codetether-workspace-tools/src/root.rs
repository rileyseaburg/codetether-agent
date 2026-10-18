//! Workspace-root resolution for local swarm participants.

use std::path::PathBuf;

/// Select the configured directory, current directory, or a relative fallback.
/// # Arguments
/// * `configured` — Optional caller-provided workspace path.
/// # Returns
/// An owned path without canonicalizing or requiring the directory to exist.
/// # Examples
/// ```
/// assert_eq!(codetether_workspace_tools::resolve_root(Some("workspace")), std::path::PathBuf::from("workspace"));
/// ```
pub fn resolve_root(configured: Option<&str>) -> PathBuf {
    configured
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}
