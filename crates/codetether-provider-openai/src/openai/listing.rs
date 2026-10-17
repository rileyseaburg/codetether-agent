//! Discover provider models with a static fallback catalog.

use super::*;
impl OpenAIProvider {
    pub(super) async fn list_provider_models(&self) -> Result<Vec<ModelInfo>> {
        // For non-OpenAI providers, return provider-specific model defaults.
        // Note: async-openai 0.32 does not expose a stable models list API across
        // all OpenAI-compatible endpoints.
        if self.provider_name != "openai" {
            let discovered = self.discover_models_from_api().await;
            if !discovered.is_empty() {
                return Ok(discovered);
            }
            return Ok(self.provider_default_models());
        }

        // OpenAI default models
        Ok(openai_models::openai_default_models())
    }
}
