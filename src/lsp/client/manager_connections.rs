//! manager connections operations for language server clients.

use super::*;

impl LspManager {
    /// Get or create a client for the given language
    pub async fn get_client(&self, language: &str) -> Result<Arc<LspClient>> {
        let mut clients = self.clients.write().await;
        if let Some(client) = clients.get(language) {
            return Ok(client.touch());
        }

        let client = if let Some(settings) = &self.lsp_settings {
            if let Some(entry) = settings.servers.get(language) {
                let config = LspConfig::from_server_entry(entry, self.root_uri.clone());
                LspClient::new(config).await?
            } else {
                LspClient::for_language(language, self.root_uri.clone()).await?
            }
        } else {
            LspClient::for_language(language, self.root_uri.clone()).await?
        };
        client.initialize().await?;

        let client = Arc::new(client);
        clients.insert(language.to_string(), Arc::clone(&client));

        Ok(client)
    }

    /// Get a client for a file path (detects language from extension)
    pub async fn get_client_for_file(&self, path: &Path) -> Result<Arc<LspClient>> {
        let language = detect_language_from_path(path.to_string_lossy().as_ref())
            .ok_or_else(|| anyhow::anyhow!("Unknown language for file: {}", path.display()))?;
        self.get_client(language).await
    }
}
