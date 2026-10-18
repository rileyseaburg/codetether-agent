//! MCP wrappers bound to the application's existing authorized client.
/// Remote tool wrapper using the application's policy-aware MCP client.
///
/// # Examples
/// ```rust,no_run
/// # fn example(client: std::sync::Arc<codetether_native_mcp_client::McpClient>, tool: codetether_native_mcp_client::McpTool) {
/// let _wrapper = codetether_native_mcp_client::McpToolWrapper::new(client, tool);
/// # }
/// ```
pub type McpToolWrapper = codetether_mcp_tools::McpToolWrapper<crate::McpClient>;
/// Connection manager using the application's policy-aware MCP client.
///
/// # Examples
/// ```rust,no_run
/// # async fn example(manager: codetether_native_mcp_client::McpToolManager) {
/// let _wrappers = manager.wrappers().await;
/// # }
/// ```
pub type McpToolManager = codetether_mcp_tools::McpToolManager<crate::McpClient>;
