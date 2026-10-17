//! Catalog get provider.
use super::*;
impl ModelCatalog {
    /// Get a provider by ID
    pub fn get_provider(&self, provider_id: &str) -> Option<&ProviderInfo> {
        self.providers.get(provider_id)
    }
}
