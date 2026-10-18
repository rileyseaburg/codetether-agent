//! Return connection summaries as a JSON array.
use super::{ConnectionClient, Connections};
use serde_json::{Value, json};
impl<C: ConnectionClient> Connections<C> {
    /// Return connection summaries as a JSON array.
    /// # Arguments
    /// Uses the current connection snapshot.
    /// # Returns
    /// Server name, command and tool-count objects.
    /// # Examples
    /// ```no_run
    /// # async fn example<C: codetether_mcp_connections::ConnectionClient>(registry: &codetether_mcp_connections::Connections<C>) -> anyhow::Result<()> {
    /// assert!(registry.summary_json().await.is_array());
    /// # Ok(())
    /// # }
    /// ```
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
