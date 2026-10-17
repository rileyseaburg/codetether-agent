//! Resolve a workspace assignment to its worker bus.
use super::*;

///
/// # Arguments
/// The control-plane base URL, lookup identity when applicable, and optional bearer token.
///
/// # Returns
/// The resolved endpoint or workspace identity.
///
/// # Errors
/// Returns an error for failed HTTP requests, invalid responses or an unresolved assignment.
///
/// # Examples
/// ```rust,no_run
/// async fn lookup() -> anyhow::Result<()> {
///     let _result = codetether_bus_http::resolve_worker_bus_url_for_workspace("http://localhost", "workspace", None).await?;
///     Ok(())
/// }
/// ```
pub async fn resolve_worker_bus_url_for_workspace(
    control_plane_url: &str,
    workspace_id: &str,
    token: Option<&str>,
) -> anyhow::Result<String> {
    let workspace_url = format!(
        "{}/v1/agent/workspaces/{}",
        control_plane_url.trim_end_matches('/'),
        urlencoding::encode(workspace_id)
    );

    let client = reqwest::Client::new();
    let mut req = client.get(workspace_url);
    if let Some(token) = token.filter(|value| !value.trim().is_empty()) {
        req = req.bearer_auth(token);
    }

    let workspace: WorkspaceDetails = req.send().await?.error_for_status()?.json().await?;
    let worker_id = workspace
        .worker_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Workspace '{}' is not currently assigned to a worker; pass --worker-id or register the workspace on a worker",
                workspace_id
            )
        })?;

    resolve_worker_bus_url(control_plane_url, worker_id, token).await
}
