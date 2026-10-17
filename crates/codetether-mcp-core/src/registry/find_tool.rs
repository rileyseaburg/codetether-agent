//! Generic MCP registry find_tool operations.
use super::*;
impl<C: RegistryClient> ServerRegistry<C> {
    /// Find a specific tool across all servers
    pub async fn find_tool(&self, tool_name: &str) -> Option<(String, McpTool)> {
        // First check the tool index
        if let Some(server_name) = self.tool_index.read().await.get(tool_name)
            && let Some(client) = self.get(server_name).await
            && let Some(tool) = client.tools().await.iter().find(|t| t.name == tool_name)
        {
            return Some((server_name.clone(), tool.clone()));
        }

        // Fallback: search all clients
        for (name, client) in self.clients.read().await.iter() {
            if let Some(tool) = client.tools().await.iter().find(|t| t.name == tool_name) {
                // Update index
                self.tool_index
                    .write()
                    .await
                    .insert(tool_name.to_string(), name.clone());
                return Some((name.clone(), tool.clone()));
            }
        }

        None
    }
}
