//! Workspace registration for clone-repo tasks.

use std::path::Path;

use anyhow::{Context, Result};
use reqwest::Client;

use super::RegisteredWorkspaceRecord;

/// Register a cloned checkout with the control plane.
///
/// # Arguments
/// * `client` - HTTP client for the request.
/// * `server` - Control-plane base URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Worker to associate with the checkout.
/// * `workspace` - Original server workspace metadata.
/// * `repo_path` - Local checkout path to register.
/// # Returns
/// Unit when the server accepts the registration.
/// # Errors
/// Returns HTTP transport errors or rejected registration responses.
/// # Examples
/// ```rust,no_run
/// # async fn example(workspace: &codetether_a2a_http::workspace::RegisteredWorkspaceRecord) -> anyhow::Result<()> {
/// use codetether_a2a_http::workspace::register_cloned_workspace;
/// register_cloned_workspace(&reqwest::Client::new(), "http://localhost:8001", &None, "worker", workspace, std::path::Path::new("repo")).await?;
/// # Ok(()) }
/// ```
pub async fn register_cloned_workspace(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    workspace: &RegisteredWorkspaceRecord,
    repo_path: &Path,
) -> Result<()> {
    let mut request = client.post(format!(
        "{}/v1/agent/workspaces",
        server.trim_end_matches('/')
    ));
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    let response = request.json(&serde_json::json!({ "workspace_id": workspace.id, "name": workspace.name, "path": repo_path.display().to_string(), "description": workspace.description, "agent_config": workspace.agent_config, "git_url": workspace.git_url, "git_branch": workspace.git_branch, "worker_id": worker_id })).send().await.context("Failed to register cloned workspace with server")?;
    if response.status().is_success() {
        return Ok(());
    }
    anyhow::bail!(
        "Failed to register cloned workspace {} ({}): {}",
        workspace.id,
        response.status(),
        response.text().await.unwrap_or_default()
    )
}
