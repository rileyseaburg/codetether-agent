//! Catalog all providers.
use super::*;
impl ModelCatalog {
    /// Get iterator over all providers and their info (unfiltered, no API key check)
    pub fn all_providers(&self) -> &HashMap<String, ProviderInfo> {
        &self.providers
    }
}
