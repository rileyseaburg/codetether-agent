//! Catalog get model.
use super::*;
impl ModelCatalog {
    /// Get a model by provider and model ID
    pub fn get_model(&self, provider_id: &str, model_id: &str) -> Option<&ApiModelInfo> {
        self.providers
            .get(provider_id)
            .and_then(|p| p.models.get(model_id))
    }
}
