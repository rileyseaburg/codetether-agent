//! Adapts a remote MCP tool to the local Tool execution interface.

use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

/// Wraps a single remote MCP tool so it can be executed via the local Tool trait.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn example(client: std::sync::Arc<codetether_agent::mcp::McpClient>, tool: codetether_agent::mcp::McpTool) {
/// use codetether_agent::tool::{Tool, McpToolWrapper};
/// let wrapper = McpToolWrapper::new(client, tool);
/// assert!(wrapper.id().starts_with("mcp:"));
/// # }
/// ```
///
/// Execution preserves remote errors and image metadata for the runtime.
#[derive(Clone)]
pub struct McpToolWrapper {
    client: Arc<crate::mcp::McpClient>,
    tool: crate::mcp::McpTool,
    id: String,
}

impl McpToolWrapper {
    /// Construct a wrapper using the remote definition and shared client.
    pub fn new(client: Arc<crate::mcp::McpClient>, tool: crate::mcp::McpTool) -> Self {
        let id = format!("mcp:{}", tool.name);
        Self { client, tool, id }
    }
}

#[async_trait]
impl Tool for McpToolWrapper {
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
