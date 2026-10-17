//! Discovery orchestration and diagnostics.
use crate::*;
impl CopilotProvider {
    /// Discover models dynamically from the Copilot /models API endpoint.
    pub(crate) async fn discover_models_from_api(&self) -> Vec<ModelInfo> {
        let Some(response) = self.discovery_send().await else {
            return Vec::new();
        };

        let Some(parsed) = self.discovery_decode(response).await else {
            return Vec::new();
        };

        let models: Vec<ModelInfo> = parsed
            .data
            .into_iter()
            .filter(|model| filter::enabled(model))
            .map(|model| self.discovery_model(model))
            .collect();

        tracing::info!(
            provider = %self.provider_name,
            count = models.len(),
            "Discovered models from Copilot API"
        );
        models
    }
}
mod decode;
mod filter;
mod model;
mod send;
