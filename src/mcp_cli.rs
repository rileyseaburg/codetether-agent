//! Native local-tool and authorized client binding for MCP CLI commands.
use crate::{cli, mcp, tool};
struct NativeHost;
#[async_trait::async_trait(?Send)]
impl codetether_mcp_cli::Host for NativeHost {
    type Client = mcp::McpClient;
    async fn builtin_tools() -> Vec<mcp::ToolMetadata> {
        let registry = std::sync::Arc::new(tool::ToolRegistry::with_defaults());
        let server = mcp::McpServer::new_local().with_tool_registry(registry);
        server.setup_tools_public().await;
        server.get_all_tool_metadata().await
    }
    async fn builtin_call(
        tool: &str,
        arguments: serde_json::Value,
    ) -> anyhow::Result<mcp::CallToolResult> {
        let registry = std::sync::Arc::new(tool::ToolRegistry::with_defaults());
        let server = mcp::McpServer::new_local().with_tool_registry(registry);
        server.setup_tools_public().await;
        server.call_tool_direct(tool, arguments).await
    }
}
pub(super) async fn execute(
    args: cli::McpArgs,
    command: Option<(String, Vec<String>)>,
) -> anyhow::Result<()> {
    codetether_mcp_cli::execute::<NativeHost>(args, command).await
}
