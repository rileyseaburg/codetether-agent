//! manager linters operations for language server clients.

use super::*;

impl<C: ManagedClient> LspManager<C> {
    /// Get or start a linter client by name (e.g. "eslint", "ruff").
    /// Returns `None` if the linter is not configured or its binary is missing.
    pub async fn get_linter_client(&self, name: &str) -> Result<Option<Arc<C>>> {
        // Already running?
        let mut linters = self.linter_clients.write().await;
        if let Some(client) = linters.get(name) {
            return Ok(Some(crate::client_cache::touch(client)));
        }

        let lsp_config = self.resolve_linter_config(name);

        let Some(config) = lsp_config else {
            return Ok(None);
        };

        // Try to start; if the binary is missing, return None instead of hard error
        let client = match C::new(config).await {
            Ok(c) => c,
            Err(e) => {
                debug!(target: "codetether_agent::lsp::client::manager_linters", linter = name, error = %e, "Linter server not available");
                return Ok(None);
            }
        };
        if let Err(e) = client.initialize().await {
            warn!(target: "codetether_agent::lsp::client::manager_linters", linter = name, error = %e, "Linter server failed to initialize");
            return Ok(None);
        }

        let client = Arc::new(client);
        linters.insert(name.to_string(), Arc::clone(&client));
        info!(target: "codetether_agent::lsp::client::manager_linters", linter = name, "Linter server started");
        Ok(Some(client))
    }
}
