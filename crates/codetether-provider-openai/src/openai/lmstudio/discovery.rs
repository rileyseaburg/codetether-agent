//! Fresh loaded-instance metadata, separate from queue and request budgeting.
use super::*;

impl OpenAIProvider {
    pub(in crate::openai) async fn lmstudio_models(&self) -> Result<Vec<ModelInfo>> {
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
}
