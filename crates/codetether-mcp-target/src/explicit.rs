//! Resolve explicitly named workers and workspaces.
pub(super) async fn worker(
    worker_id: &str,
    mcp_control_plane_url: &Option<String>,
    mcp_control_plane_token: &Option<String>,
) -> anyhow::Result<Option<String>> {
    let control_plane_url = mcp_control_plane_url.as_deref().ok_or_else(|| {
                            anyhow::anyhow!(
                                "--worker-id requires --server (or CODETETHER_SERVER) so MCP can resolve the worker connection"
                            )
                        })?;
    Ok(Some(
        codetether_bus_http::resolve_worker_bus_url(
            control_plane_url,
            worker_id,
            mcp_control_plane_token.as_deref(),
        )
        .await?,
    ))
}
pub(super) async fn workspace(
    workspace_id: &str,
    mcp_control_plane_url: &Option<String>,
    mcp_control_plane_token: &Option<String>,
) -> anyhow::Result<Option<String>> {
    let control_plane_url = mcp_control_plane_url.as_deref().ok_or_else(|| {
                            anyhow::anyhow!(
                                "--workspace-id requires --server (or CODETETHER_SERVER) so MCP can resolve the owning worker"
                            )
                        })?;
    Ok(Some(
        codetether_bus_http::resolve_worker_bus_url_for_workspace(
            control_plane_url,
            workspace_id,
            mcp_control_plane_token.as_deref(),
        )
        .await?,
    ))
}
