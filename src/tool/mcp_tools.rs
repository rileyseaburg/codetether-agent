//! MCP tool wrappers for exposing external MCP tools through the local Tool trait.

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

#[path = "mcp_tools_connect.rs"]
mod connect;
#[path = "mcp_tools_manager.rs"]
mod manager;
#[path = "mcp_tools_wrapper.rs"]
mod wrapper;
#[path = "mcp_tools_wrapper_execute.rs"]
mod wrapper_execute;

/// Manages a connection to an MCP server and produces local tool wrappers.
pub struct McpToolManager {
    client: Arc<crate::mcp::McpClient>,
    authority: String,
}

/// Wraps a single remote MCP tool so it can be executed via the local Tool trait.
#[derive(Clone)]
pub struct McpToolWrapper {
    client: Arc<crate::mcp::McpClient>,
    tool: crate::mcp::McpTool,
    id: String,
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
        wrapper_execute::run(self, args).await
    }
}
