//! manager lifecycle operations for language server clients.

use super::*;

impl<C: ManagedClient> LspManager<C> {
    /// Create a new LSP manager
    pub fn new(root_uri: Option<String>) -> Self {
        Self {
            clients: super::super::client_cache::ClientCache::default(),
            linter_clients: super::super::client_cache::ClientCache::default(),
            root_uri,
            lsp_settings: None,
        }
    }

    /// Create a new LSP manager with config-driven settings.
    pub fn with_config(root_uri: Option<String>, settings: codetether_config::LspSettings) -> Self {
        Self {
            clients: super::super::client_cache::ClientCache::default(),
            linter_clients: super::super::client_cache::ClientCache::default(),
            root_uri,
            lsp_settings: Some(settings),
        }
    }

    /// Shutdown all clients
    #[allow(dead_code)]
    pub async fn shutdown_all(&self) {
        let clients = self.clients.read().await;
        for (lang, client) in clients.iter() {
            if let Err(e) = client.shutdown().await {
                warn!(target: "codetether_agent::lsp::client::manager_lifecycle", "Failed to shutdown {} language server: {}", lang, e);
            }
        }
        let linters = self.linter_clients.read().await;
        for (name, client) in linters.iter() {
            if let Err(e) = client.shutdown().await {
                warn!(target: "codetether_agent::lsp::client::manager_lifecycle", "Failed to shutdown {} linter server: {}", name, e);
            }
        }
    }
}
