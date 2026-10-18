//! Connect a named server, replacing an existing client with the same name.
use super::Connection;
use super::{ConnectionClient, Connections};
use anyhow::{Result, anyhow};
impl<C: ConnectionClient> Connections<C> {
    /// Connect a named server, replacing an existing client with the same name.
    /// # Errors
    /// Returns the existing client error or a missing-server/invalid-command error.
    /// # Arguments
    /// * `name` - Registry key.
    /// * `command` - Whitespace-separated subprocess command and arguments.
    /// # Returns
    /// Current tool count of the connected server.
    /// # Examples
    /// ```no_run
    /// # async fn example<C: codetether_mcp_connections::ConnectionClient>(registry: &codetether_mcp_connections::Connections<C>) -> anyhow::Result<()> {
    /// let _ = registry.connect("files", "files-server").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(&self, name: &str, command: &str) -> Result<usize> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(anyhow!("Empty MCP command"));
        }

        let client = C::connect_subprocess(parts[0], &parts[1..]).await?;
        let tool_count = client.tools().await.len();

        let mut connections = self.connections.write().await;
        if let Some(existing) = connections.iter_mut().find(|conn| conn.name == name) {
            existing.command = command.to_string();
            existing.client = client;
            return Ok(tool_count);
        }

        connections.push(Connection {
            name: name.to_string(),
            command: command.to_string(),
            client,
        });
        Ok(tool_count)
    }
}
