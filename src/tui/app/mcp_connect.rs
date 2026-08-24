//! Approval-aware connection of a TUI MCP subprocess.

use super::{TuiMcpConnection, TuiMcpRegistry};
use anyhow::{Result, anyhow};

impl TuiMcpRegistry {
    pub async fn connect(
        &self,
        name: &str,
        command: &str,
        approval_id: Option<&str>,
        network_allowed: bool,
        session_id: &str,
    ) -> Result<usize> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(anyhow!("Empty MCP command"));
        }
        let client = crate::mcp::McpClient::connect_subprocess_with_runtime(
            parts[0],
            &parts[1..],
            approval_id,
            network_allowed,
            session_id,
        )
        .await?;
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
}
