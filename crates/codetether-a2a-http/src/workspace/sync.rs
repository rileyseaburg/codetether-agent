//! Server workspace list synchronization.

use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;
use tokio::sync::Mutex;

/// Synchronize existing local workspace paths from the control-plane catalog.
///
/// # Arguments
/// * `client` - HTTP client used for catalog retrieval.
/// * `server` - Control-plane base URL.
/// * `token` - Optional bearer token.
/// * `shared_codebases` - Shared list of local checkout paths to extend.
/// * `configure_entry` - Repository setup invoked for every returned entry.
/// # Returns
/// Unit after adding newly discovered existing paths, or after a non-success response.
/// # Errors
/// Returns HTTP transport or response JSON errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use std::sync::Arc;
/// use tokio::sync::Mutex;
/// use codetether_a2a_http::workspace::sync_workspaces_from_server;
/// let paths = Arc::new(Mutex::new(Vec::new()));
/// sync_workspaces_from_server(&reqwest::Client::new(), "http://localhost:8001", &None, &paths, |_| {}).await?;
/// # Ok(()) }
/// ```
pub async fn sync_workspaces_from_server(
    client: &Client,
    server: &str,
    token: &Option<String>,
    shared_codebases: &Arc<Mutex<Vec<String>>>,
    mut configure_entry: impl FnMut(&serde_json::Value),
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
    let entries = super::catalog::workspace_entries(data);
    let current = shared_codebases.lock().await.clone();
    let new_paths: Vec<String> = entries
        .iter()
        .filter_map(|entry| {
            configure_entry(entry);
            let path = entry["path"].as_str()?.trim();
            (!path.is_empty()
                && std::path::Path::new(path).exists()
                && !current.iter().any(|c| c == path))
            .then(|| path.to_string())
        })
        .collect();
    if new_paths.is_empty() {
        return Ok(());
    }
    let mut current = shared_codebases.lock().await;
    for path in &new_paths {
        tracing::info!(target: "codetether_agent::a2a::worker::workspace_sync", path = %path, "Workspace sync: auto-discovered local path, adding to codebases");
        current.push(path.clone());
    }
    tracing::info!(
        added = new_paths.len(),
        total = current.len(),
        "Workspace sync complete -- new paths take effect on next reconnect"
    );
    Ok(())
}
