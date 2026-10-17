//! Generic MCP registry disconnect operations.
use super::*;
impl<C: RegistryClient> ServerRegistry<C> {
    /// Disconnect from a server
    pub async fn disconnect(&self, name: &str) -> Result<()> {
        if let Some(client) = self.clients.write().await.remove(name) {
            // Remove capabilities
            self.server_capabilities.write().await.remove(name);
            // Remove from tool index
            let mut tool_index = self.tool_index.write().await;
            tool_index.retain(|_, server| server != name);
            // Close connection
            client.close().await?;
        }
        Ok(())
    }
}
