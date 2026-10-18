//! Default worker discovery for project fallbacks.
pub(super) async fn resolve(
    control_plane_url: &str,
    mcp_control_plane_token: &Option<String>,
) -> anyhow::Result<Option<String>> {
    Ok(Some(
        codetether_bus_http::resolve_default_worker_bus_url(
            control_plane_url,
            mcp_control_plane_token.as_deref(),
        )
        .await?,
    ))
}
