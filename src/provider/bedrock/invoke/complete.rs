//! Non-streaming InvokeModel completion dispatch with retry handling.

use crate::provider::bedrock::invoke::body::build_anthropic_messages_body;
use crate::provider::bedrock::invoke::error::invoke_error;
use crate::provider::bedrock::invoke::response::parse_anthropic_messages_response;
use crate::provider::bedrock::{BedrockProvider, retry};
use crate::provider::{CompletionRequest, CompletionResponse};
use anyhow::{Context, Result};

impl BedrockProvider {
    /// Complete a request via the Bedrock InvokeModel API.
    ///
    /// Retries transient throttling/5xx failures with exponential backoff +
    /// jitter, matching the Converse path's [`retry::RetryPolicy`].
    pub(in crate::provider::bedrock) async fn complete_invoke_model(
        &self,
        request: &CompletionRequest,
        model_id: &str,
    ) -> Result<CompletionResponse> {
        let body = build_anthropic_messages_body(request, model_id);
        let body_bytes = serde_json::to_vec(&body)?;
        let url = format!("{}/model/{}/invoke", self.base_url(), model_id);
        let policy = retry::RetryPolicy::default();

        for attempt in 1..=policy.max_attempts {
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
            if retry::should_retry_status(status.as_u16()) && attempt < policy.max_attempts {
                tokio::time::sleep(policy.delay_for(attempt)).await;
                continue;
            }
            return Err(invoke_error(status, &text, model_id, self.region()));
        }
        unreachable!("retry loop exits via return");
    }
}
