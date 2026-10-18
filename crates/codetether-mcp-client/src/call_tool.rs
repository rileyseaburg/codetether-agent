//! Native policy and transport bindings for shared outbound MCP enforcement.
use super::{Application, CallToolResult, McpClient};
use anyhow::Result;
use serde_json::Value;
impl<H: Application> codetether_mcp_approval::outbound::Host for McpClient<H> {
    async fn blocked(&self, name: &str, arguments: &Value) -> Option<String> {
        H::blocked(name, arguments).await
    }
    async fn call(&self, name: &str, arguments: Value) -> Result<CallToolResult> {
        self.inner.connection().call_tool(name, arguments).await
    }
}
impl<H: Application> McpClient<H> {
    pub(super) async fn call_tool_checked(
        &self,
        name: &str,
        arguments: Value,
    ) -> Result<CallToolResult> {
        codetether_mcp_approval::outbound::call_checked(self, name, arguments).await
    }
}
