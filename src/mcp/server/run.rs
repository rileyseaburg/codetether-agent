//! Native entry point for the shared MCP receive loop.
use super::McpServer;
impl McpServer {
    /// Run the MCP server until transport closure.
    pub async fn run(&self) -> anyhow::Result<()> {
        codetether_mcp_server::run(self).await
    }
}
