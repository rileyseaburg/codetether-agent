//! Loaded LM Studio capacity, never the architecture's advertised maximum.
//!
//! Discovery and each chat submission read fresh instance metadata. No global
//! model-ID overrides are installed, so another provider serving the same model
//! keeps its own capacity. Unknown/unloaded capacity fails closed.

mod budget;
mod metadata;

use super::*;
use anyhow::Context;

impl OpenAIProvider {
    pub(super) fn is_lmstudio(&self) -> bool {
        matches!(self.provider_name.as_str(), "lmstudio" | "lm-studio")
    }

    pub(super) async fn lmstudio_models(&self) -> Result<Vec<ModelInfo>> {
        let root = self.api_base.strip_suffix("/v1").unwrap_or(&self.api_base);
        let url = format!("{root}/api/v0/models");
        let mut request = self.http.get(url);
        if let Some(key) = &self.api_key {
            request = request.bearer_auth(key);
        }
        let response = request
            .send()
            .await
            .context("LM Studio loaded-context discovery failed; refusing guessed capacity")?
            .error_for_status()
            .context("LM Studio /api/v0/models is required for safe context sizing")?;
        let payload = codetether_provider_core::body_cap::json_capped::<serde_json::Value>(
            response,
            codetether_provider_core::body_cap::PROVIDER_METADATA_BODY_CAP,
        )
        .await
        .context("Invalid LM Studio loaded-context metadata")?;
        metadata::models(&payload, &self.provider_name)
    }

    pub(super) async fn guard_lmstudio_request(
        &self,
        mut request: CompletionRequest,
    ) -> Result<CompletionRequest> {
        if !self.is_lmstudio() {
            return Ok(request);
        }
        let models = self.lmstudio_models().await?;
        let model = models.iter().find(|model| model.id == request.model)
            .with_context(|| format!(
                "LM Studio model '{}' has no loaded_context_length; load the model before sending a request",
                request.model,
            ))?;
        budget::cap_output(&mut request, model.context_window)?;
        Ok(request)
    }
}
