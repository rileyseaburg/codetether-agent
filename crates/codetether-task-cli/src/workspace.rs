use crate::{matching::best_workspace_match, types::WorkspaceSummary};
use anyhow::{Context, Result};
use reqwest::Client;
use std::path::Path;

pub(crate) async fn resolve_workspace_id(
    client: &Client,
    server: &str,
    token: Option<&str>,
    workspace_root: &Path,
) -> Result<String> {
    let workspace_root = workspace_root.to_string_lossy().to_string();
    let mut req = client.get(format!("{}/v1/agent/workspaces", server));
    if let Some(token) = token {
        req = req.bearer_auth(token);
    }

    let response = req
        .send()
        .await
        .context("Failed to list workspaces from CodeTether server")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Workspace lookup failed ({}): {}", status, body);
    }

    let workspaces: Vec<WorkspaceSummary> = response
        .json()
        .await
        .context("Failed to parse workspace list")?;

    match best_workspace_match(&workspace_root, &workspaces) {
        Some(workspace) => Ok(workspace.id.clone()),
        None => anyhow::bail!(
            "No registered workspace matched '{}'. Register this workspace first or pass --workspace-id.",
            workspace_root
        ),
    }
}
