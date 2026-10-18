//! Explicit bus, worker and workspace selection.
/// Resolve the bus URL before attaching an MCP server.
/// # Arguments
/// * `args` - MCP target options, in priority order.
/// * `mcp_control_plane_url` - Optional control-plane server.
/// * `mcp_control_plane_token` - Optional authentication token.
/// * `mcp_project_root` - Explicit project path, if any.
/// * `mcp_project_was_explicit` - Whether a missing project match is an error.
/// # Returns
/// The chosen bus URL, or no attachment when no target is configured.
/// # Errors
/// Returns missing-server, project discovery or control-plane request errors.
/// # Examples
/// ```no_run
/// # async fn example(args: codetether_cli_options::McpArgs) -> anyhow::Result<()> {
/// let bus = codetether_mcp_target::resolve(&args, &None, &None, &None, false).await?;
/// # Ok(()) }
/// ```
pub async fn resolve(
    args: &codetether_cli_options::McpArgs,
    mcp_control_plane_url: &Option<String>,
    mcp_control_plane_token: &Option<String>,
    mcp_project_root: &Option<std::path::PathBuf>,
    mcp_project_was_explicit: bool,
) -> anyhow::Result<Option<String>> {
    Ok(if let Some(bus_url) = &args.bus_url {
        Some(bus_url.clone())
    } else if let Some(worker_id) = &args.worker_id {
        crate::explicit::worker(worker_id, mcp_control_plane_url, mcp_control_plane_token).await?
    } else if let Some(workspace_id) = &args.workspace_id {
        crate::explicit::workspace(workspace_id, mcp_control_plane_url, mcp_control_plane_token)
            .await?
    } else if let Some(control_plane_url) = mcp_control_plane_url.as_deref() {
        crate::project::resolve(
            control_plane_url,
            mcp_control_plane_token,
            mcp_project_root,
            mcp_project_was_explicit,
        )
        .await?
    } else {
        None
    })
}
