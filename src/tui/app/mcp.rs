//! Native authorized MCP client adapter for the shared connection registry.
use crate::mcp::McpClient;
use anyhow::Result;
use std::sync::Arc;
/// Named connections using the native authorized MCP client.
pub type TuiMcpRegistry = codetether_mcp_connections::Connections<McpClient>;
/// Server metadata returned by the native MCP connection registry.
pub use codetether_mcp_connections::ServerSummary as TuiMcpServerSummary;
impl codetether_mcp_connections::ConnectionClient for McpClient {
    async fn connect_subprocess(command: &str, args: &[&str]) -> Result<Arc<Self>> {
        McpClient::connect_subprocess(command, args).await
    }
}
