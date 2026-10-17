//! Generic MCP registry lookup operations.
use super::*;
impl<C: RegistryClient> ServerRegistry<C> {
    /// Get a connected client
    pub async fn get(&self, name: &str) -> Option<Arc<C>> {
        self.clients.read().await.get(name).cloned()
    }

    /// List all connected servers
    pub async fn list(&self) -> Vec<String> {
        self.clients.read().await.keys().cloned().collect()
    }

    /// Get capabilities for a specific server
    pub async fn get_capabilities(&self, name: &str) -> Option<ServerCapabilities> {
        self.server_capabilities.read().await.get(name).cloned()
    }
}
