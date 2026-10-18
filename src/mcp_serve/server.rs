//! Native stdio transport assembly, bus attachment and execution.
use super::host::Native;
use codetether_agent::{bus::AgentBus, mcp::McpServer, tool::ToolRegistry};
use std::sync::Arc;
impl codetether_mcp_cli::serve::Host for Native {
    type Server = McpServer;
    fn server(tools: Arc<ToolRegistry>, bus: Arc<AgentBus>) -> McpServer {
        McpServer::new_stdio()
            .with_tool_registry(tools)
            .with_agent_bus(bus)
    }
    async fn attach(server: McpServer, url: String, token: Option<String>) -> McpServer {
        server.with_bus_auth(url, token).await
    }
    async fn run(server: McpServer) -> anyhow::Result<()> {
        server.run().await
    }
}
