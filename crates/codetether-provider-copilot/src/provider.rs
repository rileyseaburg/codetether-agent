//! Provider identity, catalog selection, and unsupported streaming.
use crate::*;
impl CopilotProvider {
    pub(crate) async fn models_inner(&self) -> Result<Vec<ModelInfo>> {
        let mut models = self.discover_models_from_api().await;

        // If API discovery returned nothing, fall back to known models
        if models.is_empty() {
            tracing::info!(provider = %self.provider_name, "No models from API, using known model catalog");
            models = self.known_models();
        }

        // Enrich with pricing metadata from known premium request multipliers
        self.enrich_with_pricing(&mut models);

        // Filter out non-chat models (embeddings, etc.) and legacy dated variants
        models.retain(|m| {
            !m.id.starts_with("text-embedding")
                && !m.id.contains("-embedding-")
                && !catalog::filter::is_dated_model_variant(&m.id)
        });

        // Deduplicate by id (API sometimes returns duplicates)
        let mut seen = std::collections::HashSet::new();
        models.retain(|m| seen.insert(m.id.clone()));

        Ok(models)
    }
}
