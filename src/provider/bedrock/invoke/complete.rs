//! Non-streaming InvokeModel completion dispatch.

use crate::provider::bedrock::invoke::body::build_anthropic_messages_body;
use crate::provider::bedrock::invoke::response::parse_anthropic_messages_response;
use crate::provider::bedrock::{BedrockError, BedrockProvider};
use crate::provider::{CompletionRequest, CompletionResponse};
use anyhow::{Context, Result};

impl BedrockProvider {
    /// Complete a request via the Bedrock InvokeModel API.
    pub(in crate::provider::bedrock) async fn complete_invoke_model(
        &self,
        request: &CompletionRequest,
        model_id: &str,
    ) -> Result<CompletionResponse> {
        let body = build_anthropic_messages_body(request, model_id);
        let body_bytes = serde_json::to_vec(&body)?;
        let url = format!("{}/model/{}/invoke", self.base_url(), model_id);

        let response = self
            .send_request("POST", &url, Some(&body_bytes), "bedrock")
            .await?;
        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Bedrock InvokeModel response")?;

        if status.is_success() {
            return parse_anthropic_messages_response(&text);
        }
        tracing::debug!(
            provider = "bedrock",
            model = %model_id,
            status = %status,
            raw = %crate::util::truncate_bytes_safe(&text, 800),
            "Bedrock InvokeModel error response"
        );
        if let Ok(err) = serde_json::from_str::<BedrockError>(&text) {
            anyhow::bail!("Bedrock InvokeModel error ({status}): {}", err.message);
        }
        anyhow::bail!(
            "Bedrock InvokeModel error ({status}): {}",
            crate::util::truncate_bytes_safe(&text, 500)
        );
    }
}
