//! Catalog provider has api key.
use super::*;
impl ModelCatalog {
    /// Check if a provider has an API key configured in HashiCorp Vault
    ///
    /// NOTE: This is a sync wrapper that checks the Vault cache.
    /// For initial population, use `check_provider_api_key_async`.
    pub fn provider_has_api_key(&self, provider_id: &str) -> bool {
        // Check if we have it cached in the secrets manager
        if let Some(manager) = codetether_secrets::secrets_manager() {
            // Sync check - only works if already cached
            let cache = manager.cache.try_read();
            if let Ok(cache) = cache {
                return cache.contains_key(provider_id);
            }
        }
        false
    }
}
