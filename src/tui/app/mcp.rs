#![allow(dead_code)]

use std::sync::Arc;

use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use tokio::sync::RwLock;

use crate::mcp::{McpClient, McpTool};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TuiMcpServerSummary {
    pub name: String,
    pub command: String,
    pub tool_count: usize,
}

#[allow(dead_code)]
#[derive(Clone)]
struct TuiMcpConnection {
    name: String,
    command: String,
    client: Arc<McpClient>,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct TuiMcpRegistry {
    connections: RwLock<Vec<TuiMcpConnection>>,
}

#[allow(dead_code)]
impl TuiMcpRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn connect(&self, name: &str, command: &str) -> Result<usize> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(anyhow!("Empty MCP command"));
        }

        let client = McpClient::connect_subprocess(parts[0], &parts[1..]).await?;
        let tool_count = client.tools().await.len();

        let mut connections = self.connections.write().await;
        if let Some(existing) = connections.iter_mut().find(|conn| conn.name == name) {
            existing.command = command.to_string();
            existing.client = client;
            return Ok(tool_count);
        }

        connections.push(TuiMcpConnection {
            name: name.to_string(),
            command: command.to_string(),
            client,
        });
        Ok(tool_count)
    }

    pub async fn list_servers(&self) -> Vec<TuiMcpServerSummary> {
        let snapshot = self.connections.read().await.clone();
        let mut result = Vec::new();
        for conn in snapshot {
            let tool_count = conn.client.tools().await.len();
            result.push(TuiMcpServerSummary {
                name: conn.name,
                command: conn.command,
                tool_count,
            });
        }
        result
    }

    pub async fn list_tools(&self, server_name: Option<&str>) -> Result<Vec<(String, McpTool)>> {
        let snapshot = self.connections.read().await.clone();
        let mut result = Vec::new();
        for conn in snapshot {
            if let Some(target) = server_name
                && conn.name != target
            {
                continue;
            }
            for tool in conn.client.tools().await {
                result.push((conn.name.clone(), tool));
            }
        }

        if let Some(target) = server_name
            && result.is_empty()
        {
            return Err(anyhow!("No MCP server named '{target}'"));
        }

        Ok(result)
    }

    pub async fn call_tool(
        &self,
        server_name: &str,
        tool_name: &str,
        arguments: Value,
    ) -> Result<String> {
        let client = {
            let snapshot = self.connections.read().await;
            snapshot
                .iter()
                .find(|conn| conn.name == server_name)
                .map(|conn| Arc::clone(&conn.client))
        }
        .ok_or_else(|| anyhow!("No MCP server named '{server_name}'"))?;

        let result = client.call_tool(tool_name, arguments).await?;
        Ok(result
            .content
            .iter()
            .map(|item| match item {
                crate::mcp::ToolContent::Text { text } => text.clone(),
                crate::mcp::ToolContent::Image { data, mime_type } => {
                    format!("[image: {mime_type} ({} bytes)]", data.len())
                }
                crate::mcp::ToolContent::Resource { resource } => {
                    serde_json::to_string_pretty(resource).unwrap_or_default()
                }
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }

    pub async fn summary_json(&self) -> Value {
        let servers = self.list_servers().await;
        json!(
            servers
                .into_iter()
                .map(|server| json!({
                    "name": server.name,
                    "command": server.command,
                    "tool_count": server.tool_count,
                }))
                .collect::<Vec<_>>()
        )
    }
}
