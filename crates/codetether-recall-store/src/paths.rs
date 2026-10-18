//! Stable paths for recall sidecars and workspace catalogs.

use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

use crate::Host;

/// Canonicalize a workspace, retaining its input path when resolution fails.
/// # Arguments
/// Receives the requested workspace path.
/// # Returns
/// The canonical path when available, otherwise the original path.
/// # Examples
/// ```
/// let path = std::path::Path::new(".");
/// assert!(!codetether_recall_store::paths::canonical(path).as_os_str().is_empty());
/// ```
pub fn canonical(workspace: &Path) -> PathBuf {
    workspace
        .canonicalize()
        .unwrap_or_else(|_| workspace.to_path_buf())
}

/// Resolve a legacy sidecar path after native session-ID validation.
/// # Arguments
/// Receives the session ID and native storage host.
/// # Returns
/// A path beneath the recall sessions directory.
/// # Errors
/// Propagates native validation and root directory errors.
/// # Examples
/// ```
/// # fn example(h: &impl codetether_recall_store::Host) -> anyhow::Result<()> {
/// let path = codetether_recall_store::paths::session("session-id", h)?;
/// # Ok(())
/// # }
/// ```
pub fn session<H: Host>(session_id: &str, host: &H) -> Result<PathBuf> {
    host.session_path(session_id)?;
    Ok(host
        .root()?
        .join("sessions")
        .join(format!("{session_id}.json")))
}

/// Hash the canonical workspace into its catalog filename.
/// # Arguments
/// Receives a workspace path and native storage host.
/// # Returns
/// A SHA-256-named JSON path beneath the recall workspaces directory.
/// # Errors
/// Propagates root directory resolution errors.
/// # Examples
/// ```
/// # fn example(h: &impl codetether_recall_store::Host) -> anyhow::Result<()> {
/// let path = codetether_recall_store::paths::catalog(std::path::Path::new("."), h)?;
/// # Ok(())
/// # }
/// ```
pub fn catalog<H: Host>(workspace: &Path, host: &H) -> Result<PathBuf> {
    let workspace = canonical(workspace);
    let digest = Sha256::digest(workspace.to_string_lossy().as_bytes());
    Ok(host
        .root()?
        .join("workspaces")
        .join(format!("{}.json", hex::encode(digest))))
}
