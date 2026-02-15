//! MCP tool wrappers for exposing external MCP tools through the local Tool trait.

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

/// Manages a connection to an MCP server and produces local tool wrappers.
pub struct McpToolManager {
    client: Arc<crate::mcp::McpClient>,
}

impl McpToolManager {
    /// Connect to an MCP server subprocess and create a manager.
    pub async fn connect_subprocess(command: &str, args: &[&str]) -> Result<Self> {
        let client = crate::mcp::McpClient::connect_subprocess(command, args).await?;
        Ok(Self { client })
    }

    /// Build local wrappers for every tool currently advertised by the MCP server.
    pub async fn wrappers(&self) -> Vec<McpToolWrapper> {
        self.client
            .tools()
            .await
            .into_iter()
            .map(|tool| McpToolWrapper::new(Arc::clone(&self.client), tool))
            .collect()
    }

    /// Return the underlying MCP client.
    pub fn client(&self) -> Arc<crate::mcp::McpClient> {
        Arc::clone(&self.client)
    }
}

/// Wraps a single remote MCP tool so it can be executed via the local Tool trait.
#[derive(Clone)]
pub struct McpToolWrapper {
    client: Arc<crate::mcp::McpClient>,
    tool: crate::mcp::McpTool,
    id: String,
}

impl McpToolWrapper {
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

        let output = result
            .content
            .iter()
            .map(|item| match item {
                crate::mcp::ToolContent::Text { text } => text.clone(),
                crate::mcp::ToolContent::Image { data, mime_type } => {
                    format!("[image: {} ({} bytes)]", mime_type, data.len())
                }
                crate::mcp::ToolContent::Resource { resource } => {
                    serde_json::to_string(resource).unwrap_or_default()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        if result.is_error {
            Ok(ToolResult::error(output))
        } else {
            Ok(ToolResult::success(output))
        }
    }
}
