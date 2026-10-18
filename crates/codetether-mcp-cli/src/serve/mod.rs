//! MCP stdio serving with provider fallback and local/remote bus setup.
//!
//! [serve] creates the local event bus, selects provider-aware tools, resolves
//! remote target precedence, then runs the application-supplied MCP transport.
/// Shared provider, tool-registry and stdio-server application bindings.
pub mod application;
mod host;
mod providers;
use codetether_cli_options::McpArgs;
pub use host::{Host, ProviderHost};
use std::path::PathBuf;
/// Assemble and run the stdio MCP server.
/// # Arguments
/// Receives MCP arguments, optional control-plane URL/token, project context and
/// whether that project was explicitly selected by the user.
/// # Returns
/// Completes when the server loop ends.
/// # Errors
/// Returns target-resolution or native server execution errors.
/// # Examples
/// ```no_run
/// async fn serve<H: codetether_mcp_cli::serve::Host>(args: codetether_cli_options::McpArgs) {
///     codetether_mcp_cli::serve::serve::<H>(&args, &None, &None, &None, false).await.unwrap();
/// }
/// ```
pub async fn serve<H: Host>(
    args: &McpArgs,
    mcp_control_plane_url: &Option<String>,
    mcp_control_plane_token: &Option<String>,
    mcp_project_root: &Option<PathBuf>,
    mcp_project_was_explicit: bool,
) -> anyhow::Result<()> {
    tracing::info!("Starting MCP server over stdio...");
    let bus = codetether_bus_core::AgentBus::new().into_arc();
    codetether_bus_s3::spawn_bus_s3_sink(bus.clone());
    let registry = providers::registry::<H>().await;
    let server = H::server(registry, bus);
    let resolved_bus_url = codetether_mcp_target::resolve(
        args,
        mcp_control_plane_url,
        mcp_control_plane_token,
        mcp_project_root,
        mcp_project_was_explicit,
    )
    .await?;
    let server = if let Some(bus_url) = resolved_bus_url {
        tracing::info!(bus_url = %bus_url, "Connecting MCP server to agent bus");
        H::attach(server, bus_url, mcp_control_plane_token.clone()).await
    } else {
        server
    };
    H::run(server).await?;
    Ok(())
}
