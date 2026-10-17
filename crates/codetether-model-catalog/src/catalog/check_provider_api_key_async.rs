//! Catalog check provider api key async.
use super::*;
impl ModelCatalog {
    /// Async check if a provider has an API key in Vault
    pub async fn check_provider_api_key_async(&self, provider_id: &str) -> bool {
        codetether_secrets::has_api_key(provider_id).await
    }
}
