//! Native policy and transport bindings for shared outbound MCP enforcement.
use super::{CallToolResult, McpClient};
use anyhow::Result;
use serde_json::Value;
impl codetether_mcp_approval::outbound::Host for McpClient {
    async fn blocked(&self, name: &str, arguments: &Value) -> Option<String> {
        crate::runtime_policy::evaluate_tool_invocation(name, arguments)
            .await
            .map(|result| result.output)
    }
    async fn call(&self, name: &str, arguments: Value) -> Result<CallToolResult> {
        self.inner.connection().call_tool(name, arguments).await
    }
}
impl McpClient {
    pub(super) async fn call_tool_checked(
        &self,
        name: &str,
        arguments: Value,
    ) -> Result<CallToolResult> {
        codetether_mcp_approval::outbound::call_checked(self, name, arguments).await
    }
}
