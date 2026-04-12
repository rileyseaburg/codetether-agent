//! Workspace directory resolution for claimed worker tasks.
//!
//! This module maps a logical workspace ID to the local clone path recorded in
//! the control plane so session-based tasks run in the expected repository.
//!
//! # Examples
//!
//! ```ignore
//! let path = resolve_workspace_directory(client, server, &token, "ws-1").await?;
//! assert!(path.is_absolute());
//! ```

use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use std::path::PathBuf;

use super::clone_location::resolve_workspace_clone_path;

#[derive(Deserialize)]
struct WorkspacePathRecord {
    #[serde(default)]
    path: String,
}

/// Resolves the local checkout directory for a workspace-backed task.
///
/// The control plane is queried for the workspace path, then legacy paths are
/// normalized against the worker's configured clone root.
///
/// # Examples
///
/// ```ignore
/// let path = resolve_workspace_directory(client, server, &token, "ws-1").await?;
/// assert!(path.exists() || path.ends_with("ws-1"));
/// ```
pub(super) async fn resolve_workspace_directory(client: &Client, server: &str, token: &Option<String>, workspace_id: &str) -> Result<PathBuf> {
    let mut request = client.get(format!("{}/v1/agent/workspaces/{}", server.trim_end_matches('/'), workspace_id));
    if let Some(token) = token { request = request.bearer_auth(token); }
    let response = request.send().await.with_context(|| format!("Failed to load workspace {}", workspace_id))?;
    let response = response.error_for_status().with_context(|| format!("Failed to fetch workspace {}", workspace_id))?;
    let workspace = response.json::<WorkspacePathRecord>().await.with_context(|| format!("Failed to decode workspace {} response", workspace_id))?;
    let path = resolve_workspace_clone_path(workspace_id, &workspace.path);
    tokio::fs::metadata(&path).await.with_context(|| format!("Workspace {} resolved to missing path {}", workspace_id, path.display()))?;
    Ok(path)
}
