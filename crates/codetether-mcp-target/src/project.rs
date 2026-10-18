//! Project lookup and default-worker fallback.
pub(super) async fn resolve(
    control_plane_url: &str,
    mcp_control_plane_token: &Option<String>,
    mcp_project_root: &Option<std::path::PathBuf>,
    mcp_project_was_explicit: bool,
) -> anyhow::Result<Option<String>> {
    Ok({
        let project_root = mcp_project_root
            .clone()
            .or_else(|| std::env::current_dir().ok());
        if let Some(project_root) = project_root.as_deref() {
            if let Some(workspace_id) = codetether_bus_http::resolve_workspace_id_from_path(
                control_plane_url,
                project_root,
                mcp_control_plane_token.as_deref(),
            )
            .await?
            {
                Some(
                    codetether_bus_http::resolve_worker_bus_url_for_workspace(
                        control_plane_url,
                        &workspace_id,
                        mcp_control_plane_token.as_deref(),
                    )
                    .await?,
                )
            } else if mcp_project_was_explicit {
                anyhow::bail!(
                    "No registered workspace matched project '{}'; pass --workspace-id or --worker-id",
                    project_root.display()
                );
            } else {
                crate::default_worker::resolve(control_plane_url, mcp_control_plane_token).await?
            }
        } else {
            crate::default_worker::resolve(control_plane_url, mcp_control_plane_token).await?
        }
    })
}
