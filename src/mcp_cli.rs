//! Native registry, server policy and authorized client for shared MCP commands.
pub(super) type NativeHost = codetether_mcp_cli::application::ApplicationHost<
    crate::tool::ToolRegistry,
    crate::mcp::McpServer,
    crate::mcp::McpClient,
>;
