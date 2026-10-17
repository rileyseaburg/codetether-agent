//! Workspace routing diagnostics for worker registration.
use reqwest::Client;

/// Resolve matching workspace IDs and report routing diagnostics.
///
/// # Arguments
/// * `client` - HTTP client.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `workspace_roots` - Local roots used to filter server records.
/// # Returns
/// Matching workspace IDs, or an empty list on failure.
/// # Examples
/// ```rust,no_run
/// # async fn example() {
/// use codetether_a2a_http::workspace::resolve_and_log_workspace_ids;
/// let ids = resolve_and_log_workspace_ids(&reqwest::Client::new(), "http://localhost:8001", &None, &[]).await;
/// # }
/// ```
pub async fn resolve_and_log_workspace_ids(
    client: &Client,
    server: &str,
    token: &Option<String>,
    workspace_roots: &[String],
) -> Vec<String> {
    match super::resolve_workspace_ids(client, server, token, workspace_roots).await {
        Ok(resolved) => resolved_workspace_ids(&resolved, workspace_roots),
        Err(error) => {
            tracing::warn!(target: "codetether_agent::a2a::worker::workspace_scope", error = %error, "Failed to resolve workspace IDs from server");
            Vec::new()
        }
    }
}

fn resolved_workspace_ids(
    resolved: &[super::ResolvedWorkspace],
    workspace_roots: &[String],
) -> Vec<String> {
    if resolved.is_empty() {
        tracing::warn!(target: "codetether_agent::a2a::worker::workspace_scope", roots = ?workspace_roots, "No server-side workspace IDs resolved for configured roots — the server may not route tasks to this worker. Ensure workspaces are registered with the control plane and that their paths fall under the configured roots.");
    } else {
        let ids: Vec<String> = resolved.iter().map(|ws| ws.id.clone()).collect();
        tracing::info!(target: "codetether_agent::a2a::worker::workspace_scope", workspace_ids = ?ids, count = resolved.len(), "Resolved server-side workspace IDs for configured roots");
    }
    resolved.iter().map(|ws| ws.id.clone()).collect()
}
