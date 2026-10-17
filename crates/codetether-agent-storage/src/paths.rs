//! Safe paths for spawned-agent manifests and mailbox snapshots.

use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

/// Resolve the configured agent sidecar directory.
/// # Arguments
/// This function has no arguments.
/// # Returns
/// The configured data directory with its agents subdirectory.
/// # Errors
/// Returns an error if the data directory cannot be determined.
/// # Examples
/// ```no_run
/// # let _ = codetether_agent_storage::paths::root();
/// ```
pub fn root() -> Result<PathBuf> {
    codetether_config::Config::data_dir()
        .map(|path| path.join("agents"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
}

/// Resolve the validated manifest location for one child.
/// # Arguments
/// * `child_id` — Child session identity, at most 128 bytes.
/// # Returns
/// The child ID followed by .agent.json in the agent directory.
/// # Errors
/// Returns invalid-identity or data-directory errors.
/// # Examples
/// ```no_run
/// # let _ = codetether_agent_storage::paths::manifest("child");
/// ```
pub fn manifest(child_id: &str) -> Result<PathBuf> {
    Ok(root()?.join(format!("{}.agent.json", valid(child_id)?)))
}

/// Resolve the validated mailbox location for one child.
/// # Arguments
/// * `child_id` — Child session identity, at most 128 bytes.
/// # Returns
/// The child ID followed by .mailbox.json in the agent directory.
/// # Errors
/// Returns invalid-identity or data-directory errors.
/// # Examples
/// ```no_run
/// # let _ = codetether_agent_storage::paths::mailbox("child");
/// ```
pub fn mailbox(child_id: &str) -> Result<PathBuf> {
    Ok(root()?.join(format!("{}.mailbox.json", valid(child_id)?)))
}

/// Check a filename for the durable manifest suffix.
/// # Arguments
/// * `path` — Candidate file path.
/// # Returns
/// True for a UTF-8 filename ending in .agent.json.
/// # Examples
/// ```no_run
/// # let _ = codetether_agent_storage::paths::is_manifest(std::path::Path::new("child.agent.json"));
/// ```
pub fn is_manifest(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".agent.json"))
}

fn valid(id: &str) -> Result<&str> {
    if id.is_empty()
        || id.len() > 128
        || id.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
    {
        bail!("invalid child session id")
    }
    Ok(id)
}
