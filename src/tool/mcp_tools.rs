//! MCP wrappers bound to the application's existing authorized client.
#[path = "mcp_tools_client.rs"]
mod client;
pub(super) use codetether_mcp_tools::convert;
/// Remote tool wrapper using the application's policy-aware MCP client.
///
/// # Examples
/// ```rust,no_run
/// # fn example(client: std::sync::Arc<codetether_agent::mcp::McpClient>, tool: codetether_agent::mcp::McpTool) {
/// let _wrapper = codetether_agent::tool::McpToolWrapper::new(client, tool);
/// # }
/// ```
pub type McpToolWrapper = codetether_mcp_tools::McpToolWrapper<crate::mcp::McpClient>;
/// Connection manager using the application's policy-aware MCP client.
///
/// # Examples
/// ```rust,no_run
/// # async fn example(manager: codetether_agent::tool::McpToolManager) {
/// let _wrappers = manager.wrappers().await;
/// # }
/// ```
pub type McpToolManager = codetether_mcp_tools::McpToolManager<crate::mcp::McpClient>;
