//! List advertised tools, optionally filtering by server name.
use super::{ConnectionClient, Connections};
use anyhow::{Result, anyhow};
use codetether_mcp_core::types::McpTool;
impl<C: ConnectionClient> Connections<C> {
    /// List advertised tools, optionally filtering by server name.
    /// # Errors
    /// Returns the existing client error or a missing-server/invalid-command error.
    /// # Arguments
    /// * `server_name` - Optional registry key to select.
    /// # Returns
    /// Server names paired with advertised tool definitions.
    /// # Examples
    /// ```no_run
    /// # async fn example<C: codetether_mcp_connections::ConnectionClient>(registry: &codetether_mcp_connections::Connections<C>) -> anyhow::Result<()> {
    /// let _ = registry.list_tools(None).await?;
    /// # Ok(())
    /// # }
    /// ```
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
}
