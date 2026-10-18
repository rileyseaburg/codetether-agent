//! MCP client and registry bound to the native application's policy.
mod application;
/// Client using native process and tool authorization.
pub type McpClient = codetether_mcp_client::McpClient<application::Native>;
/// Multi-server registry using native process and tool authorization.
pub type McpRegistry = codetether_mcp_client::McpRegistry<application::Native>;
