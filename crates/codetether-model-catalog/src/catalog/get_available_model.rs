//! Catalog get available model.
use super::*;
impl ModelCatalog {
    /// Get a model only if the provider has an API key
    pub fn get_available_model(&self, provider_id: &str, model_id: &str) -> Option<&ApiModelInfo> {
        if self.provider_has_api_key(provider_id) {
            self.get_model(provider_id, model_id)
        } else {
            None
        }
    }
}
