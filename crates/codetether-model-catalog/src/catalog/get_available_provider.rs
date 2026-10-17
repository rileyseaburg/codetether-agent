//! Catalog get available provider.
use super::*;
impl ModelCatalog {
    /// Get a provider by ID only if it has an API key
    pub fn get_available_provider(&self, provider_id: &str) -> Option<&ProviderInfo> {
        if self.provider_has_api_key(provider_id) {
            self.providers.get(provider_id)
        } else {
            None
        }
    }
}
