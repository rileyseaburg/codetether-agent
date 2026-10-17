//! Catalog available providers.
use super::*;
impl ModelCatalog {
    /// Get list of providers that have API keys configured (sync, uses cache)
    pub fn available_providers(&self) -> Vec<&str> {
        self.providers
            .keys()
            .filter(|id| self.provider_has_api_key(id))
            .map(|s| s.as_str())
            .collect()
    }
}
