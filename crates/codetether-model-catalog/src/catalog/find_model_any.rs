//! Catalog find model any.
use super::*;
impl ModelCatalog {
    /// Find a model across ALL providers (ignoring API key requirement)
    pub fn find_model_any(&self, model_id: &str) -> Option<(&str, &ApiModelInfo)> {
        for (provider_id, provider) in &self.providers {
            if let Some(model) = provider.models.get(model_id) {
                return Some((provider_id, model));
            }
        }
        None
    }
}
