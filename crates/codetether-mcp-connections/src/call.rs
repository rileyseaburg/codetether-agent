//! Call a named server tool and format its content as readable text.
use super::{ConnectionClient, Connections};
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::sync::Arc;
impl<C: ConnectionClient> Connections<C> {
    /// Call a named server tool and format its content as readable text.
    /// # Errors
    /// Returns the existing client error or a missing-server/invalid-command error.
    /// # Arguments
    /// * `server_name` - Registry key.
    /// * `tool_name` - Remote tool identifier.
    /// * `arguments` - Tool input JSON.
    /// # Returns
    /// Newline-separated text, image labels and JSON resource content.
    /// # Examples
    /// ```no_run
    /// # async fn example<C: codetether_mcp_connections::ConnectionClient>(registry: &codetether_mcp_connections::Connections<C>) -> anyhow::Result<()> {
    /// let _ = registry.call_tool("files", "list", serde_json::json!({})).await?;
    /// # Ok(())
    /// # }
    /// ```
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
                codetether_mcp_core::types::ToolContent::Text { text } => text.clone(),
                codetether_mcp_core::types::ToolContent::Image { data, mime_type } => {
                    format!("[image: {mime_type} ({} bytes)]", data.len())
                }
                codetether_mcp_core::types::ToolContent::Resource { resource } => {
                    serde_json::to_string_pretty(resource).unwrap_or_default()
                }
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }
}
