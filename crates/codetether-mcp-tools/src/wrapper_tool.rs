//! Local Tool implementation forwarding through the existing MCP client.
use crate::{Client, McpToolWrapper};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<C: Client> Tool for McpToolWrapper<C> {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.tool.name
    }

    fn description(&self) -> &str {
        self.tool
            .description
            .as_deref()
            .unwrap_or("Remote MCP tool")
    }

    fn parameters(&self) -> Value {
        self.tool.input_schema.clone()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let result = self.client.call_tool(&self.tool.name, args).await?;

        Ok(super::convert::result(result))
    }
}
