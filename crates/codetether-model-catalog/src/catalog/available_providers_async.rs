//! Catalog available providers async.
use super::*;
impl ModelCatalog {
    /// Get list of providers that have API keys configured (async, checks Vault)
    #[allow(dead_code)]
    pub async fn available_providers_async(&self) -> Vec<String> {
        let mut available = Vec::new();
        for provider_id in self.providers.keys() {
            if self.check_provider_api_key_async(provider_id).await {
                available.push(provider_id.clone());
            }
        }
        available
    }
}
