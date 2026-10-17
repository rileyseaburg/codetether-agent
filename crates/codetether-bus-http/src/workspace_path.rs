//! Match a local directory against registered workspaces.
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
///     let _result = codetether_bus_http::resolve_workspace_id_from_path("http://localhost", std::path::Path::new("."), None).await?;
///     Ok(())
/// }
/// ```
pub async fn resolve_workspace_id_from_path(
    control_plane_url: &str,
    workspace_root: &Path,
    token: Option<&str>,
) -> anyhow::Result<Option<String>> {
    let workspace_root = normalize_local_path(workspace_root)?;
    let workspace_root = workspace_root.to_string_lossy().to_string();
    let workspaces_url = format!(
        "{}/v1/agent/workspaces",
        control_plane_url.trim_end_matches('/')
    );

    let client = reqwest::Client::new();
    let mut req = client.get(workspaces_url);
    if let Some(token) = token.filter(|value| !value.trim().is_empty()) {
        req = req.bearer_auth(token);
    }

    let workspaces: Vec<WorkspaceSummary> = req.send().await?.error_for_status()?.json().await?;
    Ok(best_workspace_match(&workspace_root, &workspaces).map(|workspace| workspace.id.clone()))
}
