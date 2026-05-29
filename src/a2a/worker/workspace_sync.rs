//! Server workspace list synchronization.

use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;
use tokio::sync::Mutex;

use super::workspace_sync_git::maybe_configure_repo_git_auth_from_entry;

pub(super) async fn sync_workspaces_from_server(
    client: &Client,
    server: &str,
    token: &Option<String>,
    shared_codebases: &Arc<Mutex<Vec<String>>>,
) -> Result<()> {
    let mut request = client.get(format!("{server}/v1/agent/workspaces"));
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    let response = request.send().await?;
    if !response.status().is_success() {
        return Ok(());
    }
    let data: serde_json::Value = response.json().await?;
    let entries = data
        .as_array()
        .cloned()
        .or_else(|| data["workspaces"].as_array().cloned())
        .or_else(|| data["codebases"].as_array().cloned())
        .unwrap_or_default();
    let current = shared_codebases.lock().await.clone();
    let new_paths: Vec<String> = entries.iter().filter_map(|entry| {
        maybe_configure_repo_git_auth_from_entry(entry);
        let path = entry["path"].as_str()?.trim();
        (!path.is_empty() && std::path::Path::new(path).exists() && !current.iter().any(|c| c == path)).then(|| path.to_string())
    }).collect();
    if new_paths.is_empty() {
        return Ok(());
    }
    let mut current = shared_codebases.lock().await;
    for path in &new_paths {
        tracing::info!(path = %path, "Workspace sync: auto-discovered local path, adding to codebases");
        current.push(path.clone());
    }
    tracing::info!(
        added = new_paths.len(),
        total = current.len(),
        "Workspace sync complete -- new paths take effect on next reconnect"
    );
    Ok(())
}
