//! Query the control-plane workspace catalog.
use super::*;

/// Queries the A2A server for registered workspaces under configured roots.
///
/// # Arguments
/// * `client` - HTTP client for the control plane.
/// * `server` - Control-plane base URL.
/// * `token` - Optional bearer token.
/// * `workspace_roots` - Local roots to match against returned paths.
/// # Returns
/// Nonempty workspace IDs and paths under the supplied roots; non-success
/// HTTP responses produce an empty list.
/// # Errors
/// Returns transport or JSON decoding errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_a2a_http::workspace::resolve_workspace_ids;
/// let client = reqwest::Client::new();
/// let workspaces = resolve_workspace_ids(&client, "http://localhost:8001", &None, &["/workspace".into()]).await?;
/// # Ok(()) }
/// ```
pub async fn resolve_workspace_ids(
    client: &Client,
    server: &str,
    token: &Option<String>,
    workspace_roots: &[String],
) -> Result<Vec<ResolvedWorkspace>> {
    let mut req = client.get(format!(
        "{}/v1/agent/workspaces",
        server.trim_end_matches('/')
    ));
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }
    let res = req.send().await?;
    if !res.status().is_success() {
        tracing::debug!(target: "codetether_agent::a2a::worker::workspace_resolve", status = %res.status(), "Workspace ID resolution: server returned non-success");
        return Ok(Vec::new());
    }
    let data: serde_json::Value = res.json().await?;
    Ok(workspace_entries(data)
        .into_iter()
        .filter_map(|entry| resolve_entry(&entry, workspace_roots))
        .collect())
}
