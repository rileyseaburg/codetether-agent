//! Native registry, server policy and authorized client for shared MCP commands.
pub(super) type NativeHost = codetether_mcp_cli::application::ApplicationHost<
    crate::tool::ToolRegistry,
    crate::mcp::McpServer,
    crate::mcp::McpClient,
>;
/// Provider, tool and transport identities used by MCP stdio serving.
pub(super) type ServerHost = codetether_mcp_cli::serve::application::ApplicationHost<
    crate::provider::ProviderRegistry,
    crate::tool::ToolRegistry,
    crate::mcp::McpServer,
>;
