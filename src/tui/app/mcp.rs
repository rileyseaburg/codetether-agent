//! Native authorized MCP client adapter for the shared connection registry.
use crate::mcp::McpClient;
/// Named connections using the native authorized MCP client.
pub type TuiMcpRegistry = codetether_mcp_connections::Connections<McpClient>;
/// Server metadata returned by the native MCP connection registry.
pub use codetether_mcp_connections::ServerSummary as TuiMcpServerSummary;
