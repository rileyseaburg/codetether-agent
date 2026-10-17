//! manager capabilities operations for language server clients.

use super::*;

impl<C: ManagedClient> LspManager<C> {
    /// Check if any registered client handles the given file.
    #[allow(dead_code)]
    pub async fn handles_file(&self, path: &Path) -> bool {
        let clients = self.clients.read().await;
        clients.values().any(|c| c.handles_file(path))
    }

    /// Get capabilities for a specific language server.
    #[allow(dead_code)]
    pub async fn capabilities_for(&self, language: &str) -> Option<lsp_types::ServerCapabilities> {
        let clients = self.clients.read().await;
        if let Some(client) = clients.get(language) {
            client.capabilities().await
        } else {
            None
        }
    }
}
