//! Catalog models for provider.
use super::*;
impl ModelCatalog {
    /// List models for a provider (only if API key available)
    pub fn models_for_provider(&self, provider_id: &str) -> Vec<&ApiModelInfo> {
        if !self.provider_has_api_key(provider_id) {
            return Vec::new();
        }
        self.providers
            .get(provider_id)
            .map(|p| p.models.values().collect())
            .unwrap_or_default()
    }
}
