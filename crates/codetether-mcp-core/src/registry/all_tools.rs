//! Generic MCP registry all_tools operations.
use super::*;
impl<C: RegistryClient> ServerRegistry<C> {
    /// Get all available tools from all servers
    pub async fn all_tools(&self) -> Vec<(String, McpTool)> {
        let mut all_tools = Vec::new();

        for (name, client) in self.clients.read().await.iter() {
            for tool in client.tools().await {
                all_tools.push((name.clone(), tool));
            }
        }

        all_tools
    }
}
