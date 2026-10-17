//! Catalog find model.
use super::*;
impl ModelCatalog {
    /// Find a model by ID across all providers (only available ones)
    pub fn find_model(&self, model_id: &str) -> Option<(&str, &ApiModelInfo)> {
        for (provider_id, provider) in &self.providers {
            if !self.provider_has_api_key(provider_id) {
                continue;
            }
            if let Some(model) = provider.models.get(model_id) {
                return Some((provider_id, model));
            }
        }
        None
    }
}
