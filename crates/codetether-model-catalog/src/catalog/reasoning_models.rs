//! Catalog reasoning models.
use super::*;
impl ModelCatalog {
    /// Find models with reasoning support (only from available providers)
    pub fn reasoning_models(&self) -> Vec<(&str, &ApiModelInfo)> {
        let mut result = Vec::new();
        for (provider_id, provider) in &self.providers {
            if !self.provider_has_api_key(provider_id) {
                continue;
            }
            for model in provider.models.values() {
                if model.reasoning {
                    result.push((provider_id.as_str(), model));
                }
            }
        }
        result
    }
}
