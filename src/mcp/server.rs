//! Application policy specialization of the shared MCP server.
mod application;
mod fallback;
mod registry;
/// MCP server using the native tool registry and approval policy.
pub type McpServer = codetether_mcp_server::ApplicationServer<application::Native>;
