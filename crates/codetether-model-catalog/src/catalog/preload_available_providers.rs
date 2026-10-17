//! Catalog preload available providers.
use super::*;
impl ModelCatalog {
    /// Pre-load API key availability from Vault for all providers
    pub async fn preload_available_providers(&self) -> Vec<String> {
        let mut available = Vec::new();

        if let Some(manager) = codetether_secrets::secrets_manager() {
            // List all configured providers from Vault
            if let Ok(providers) = manager.list_configured_providers().await {
                for provider_id in providers {
                    // Verify each one actually has an API key
                    if manager.has_api_key(&provider_id).await {
                        available.push(provider_id);
                    }
                }
            }
        }

        available
    }
}
