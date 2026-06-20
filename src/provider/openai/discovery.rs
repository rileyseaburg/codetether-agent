//! Live `/models` discovery for OpenAI-compatible providers.

use serde_json::Value;

use crate::provider::ModelInfo;

use super::OpenAIProvider;

impl OpenAIProvider {
    pub(super) async fn discover_models_from_api(&self) -> Vec<ModelInfo> {
        let url = format!("{}/models", self.api_base);
        let mut request = self.http.get(&url);
        if let Some(api_key) = &self.api_key {
            request = request.bearer_auth(api_key);
        }

        let response = match request.send().await {
            Ok(response) => response,
            Err(error) => {
                tracing::debug!(provider = %self.provider_name, url = %url, error = %error,
                    "Failed to fetch OpenAI-compatible /models endpoint");
                return Vec::new();
            }
        };

        let status = response.status();
        if !status.is_success() {
            tracing::debug!(provider = %self.provider_name, url = %url, status = %status,
                "OpenAI-compatible /models endpoint returned non-success");
            return Vec::new();
        }

        let payload: Value = match crate::provider::body_cap::json_capped(
            response,
            crate::provider::body_cap::PROVIDER_METADATA_BODY_CAP,
        )
        .await
        {
            Ok(payload) => payload,
            Err(error) => {
                tracing::debug!(provider = %self.provider_name, url = %url, error = %error,
                    "Failed to parse OpenAI-compatible /models response (or exceeded body cap)");
                return Vec::new();
            }
        };

        let models = Self::parse_models_payload(&payload, &self.provider_name);
        if models.is_empty() {
            tracing::debug!(provider = %self.provider_name, url = %url,
                "OpenAI-compatible /models payload did not contain any model ids");
        }
        models
    }
}
