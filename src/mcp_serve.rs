//! Bind shared MCP stdio startup to native providers, tools and transport.
mod host;
mod server;
use codetether_agent::cli::McpArgs;
use std::path::PathBuf;
pub(super) async fn serve(
    args: &McpArgs,
    url: &Option<String>,
    token: &Option<String>,
    project: &Option<PathBuf>,
    explicit: bool,
) -> anyhow::Result<()> {
    codetether_mcp_cli::serve::serve::<host::Native>(args, url, token, project, explicit).await
}
