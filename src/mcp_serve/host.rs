//! Native provider, tool and server type choices for shared MCP startup.
pub(super) type Native = codetether_mcp_cli::serve::application::ApplicationHost<
    codetether_agent::provider::ProviderRegistry,
    codetether_agent::tool::ToolRegistry,
    codetether_agent::mcp::McpServer,
>;
