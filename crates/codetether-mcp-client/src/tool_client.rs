//! Delegate shared wrapper operations to the existing authorized MCP client.
use super::{Application, CallToolResult, McpClient, McpTool};
use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
#[async_trait::async_trait]
impl<H: Application> codetether_mcp_tools::Client for McpClient<H> {
    async fn connect_subprocess_with_approval(
        command: &str,
        args: &[&str],
        approval_id: Option<&str>,
    ) -> Result<Arc<Self>> {
        McpClient::connect_subprocess_with_approval(command, args, approval_id).await
    }
    async fn tools(&self) -> Vec<McpTool> {
        McpClient::tools(self).await
    }
    async fn call_tool(&self, name: &str, args: Value) -> Result<CallToolResult> {
        McpClient::call_tool(self, name, args).await
    }
}
