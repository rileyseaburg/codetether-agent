//! Bounded live model discovery.
use crate::*;
mod model;
mod wire;
use wire::*;
impl OpenRouterProvider {
    pub(crate) async fn discover_models(&self) -> Result<Vec<ModelInfo>> {
        // Fetch models from OpenRouter API
        let response = self
            .client
            .get(format!("{}/models", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to fetch models")?;

        if !response.status().is_success() {
            return Ok(vec![]); // Return empty on error
        }

        // Cap body size: OpenRouter's /models payload is normally sub-MiB
        // but has grown over time. A hard cap prevents a runaway response
        // (or a misconfigured proxy) from OOM'ing the process during TUI
        // startup / worker registration.
        let models: ModelsResponse = match codetether_provider_core::body_cap::json_capped(
            response,
            codetether_provider_core::body_cap::PROVIDER_METADATA_BODY_CAP,
        )
        .await
        {
            Ok(v) => v,
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    "openrouter /models response exceeded body cap or failed to decode; returning empty list",
                );
                return Ok(vec![]);
            }
        };

        Ok(models.data.into_iter().map(model::info).collect())
    }
}
