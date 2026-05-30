//! Workspace registration for clone-repo tasks.

use std::path::Path;

use anyhow::{Context, Result};
use reqwest::Client;

use crate::a2a::worker_workspace_record::RegisteredWorkspaceRecord;

pub(super) async fn register_cloned_workspace(
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
