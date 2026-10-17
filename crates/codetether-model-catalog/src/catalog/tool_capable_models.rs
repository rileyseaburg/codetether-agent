//! Catalog tool capable models.
use super::*;
impl ModelCatalog {
    /// Find models with tool calling support (only from available providers)
    pub fn tool_capable_models(&self) -> Vec<(&str, &ApiModelInfo)> {
        let mut result = Vec::new();
        for (provider_id, provider) in &self.providers {
            if !self.provider_has_api_key(provider_id) {
                continue;
            }
            for model in provider.models.values() {
                if model.tool_call {
                    result.push((provider_id.as_str(), model));
                }
            }
        }
        result
    }
}
