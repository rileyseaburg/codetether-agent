//! Generic MCP registry refresh operations.
use super::*;
impl<C: RegistryClient> ServerRegistry<C> {
    /// Refresh the tool index from all servers
    pub async fn refresh_tool_index(&self) {
        let mut tool_index = self.tool_index.write().await;
        tool_index.clear();

        for (name, client) in self.clients.read().await.iter() {
            for tool in client.tools().await {
                tool_index.insert(tool.name.clone(), name.clone());
            }
        }

        tracing::info!(target: "codetether_agent::mcp::client", "Refreshed tool index with {} tools", tool_index.len());
    }
}
