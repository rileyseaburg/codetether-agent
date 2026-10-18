//! Native binding for the shared fallback MCP catalog.
mod host;
use super::McpServer;
pub(super) async fn register(server: &McpServer) {
    codetether_mcp_fallback::register::<host::Native>(&server.catalog).await;
}
