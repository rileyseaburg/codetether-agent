//! Bind the shared MCP bridge to the application's existing client and policy.
#[cfg(test)]
#[path = "mcp_bridge_policy.rs"]
mod policy;
/// MCP bridge using application authorization and resource access.
///
/// # Examples
/// ```rust
/// let _tool=codetether_native_mcp_client::mcp_bridge::McpBridgeTool::new();
/// ```
pub type McpBridgeTool = codetether_mcp_bridge::McpBridgeTool<ApplicationHost>;
/// Application policy hooks shared with all other local tools.
///
/// # Examples
/// ```rust
/// let _host=codetether_native_mcp_client::mcp_bridge::ApplicationHost;
/// ```
pub struct ApplicationHost;
#[async_trait::async_trait]
impl codetether_mcp_bridge::Host for ApplicationHost {
    type Client = crate::McpClient;
    fn approved_invocation(tool: &str, args: &serde_json::Value) -> bool {
        codetether_native_runtime_policy::approved_invocation(tool, args)
    }
    async fn evaluate_tool_invocation(
        tool: &str,
        args: &serde_json::Value,
    ) -> Option<codetether_tool_core::ToolResult> {
        codetether_native_runtime_policy::evaluate_tool_invocation(tool, args).await
    }
}
