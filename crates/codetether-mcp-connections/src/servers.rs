//! List connected servers in insertion order with current tool counts.
use super::ServerSummary;
use super::{ConnectionClient, Connections};
impl<C: ConnectionClient> Connections<C> {
    /// List connected servers in insertion order with current tool counts.
    /// # Arguments
    /// Uses the current connection snapshot.
    /// # Returns
    /// Server summaries in connection order.
    /// # Examples
    /// ```no_run
    /// # async fn example<C: codetether_mcp_connections::ConnectionClient>(registry: &codetether_mcp_connections::Connections<C>) -> anyhow::Result<()> {
    /// let _ = registry.list_servers().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_servers(&self) -> Vec<ServerSummary> {
        let snapshot = self.connections.read().await.clone();
        let mut result = Vec::new();
        for conn in snapshot {
            let tool_count = conn.client.tools().await.len();
            result.push(ServerSummary {
                name: conn.name,
                command: conn.command,
                tool_count,
            });
        }
        result
    }
}
