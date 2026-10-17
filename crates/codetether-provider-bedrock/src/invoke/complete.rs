//! Non-streaming InvokeModel completion dispatch with retry handling.

use crate::invoke::error::invoke_error;
use crate::{BedrockClient, recovery as auth_recover, retry};
use anyhow::{Context, Result};
use codetether_bedrock_protocol::native::body::build_anthropic_messages_body;
use codetether_bedrock_protocol::native::response::parse_anthropic_messages_response;
use codetether_provider_core::{CompletionRequest, CompletionResponse};

impl BedrockClient<'_> {
    /// Complete a request via the Bedrock InvokeModel API.
    ///
    /// Retries transient throttling/5xx failures with exponential backoff +
    /// jitter, matching the Converse path's [`retry::RetryPolicy`].
    ///
    /// # Arguments
    /// Accepts the request, resolved model ID, and application recovery handler.
    ///
    /// # Returns
    /// The completion result or its provider-neutral stream.
    ///
    /// # Errors
    /// Returns an error when request execution or response decoding fails.
    ///
    /// # Examples
    /// Construct [BedrockClient] and pass an implementation of
    /// [crate::recovery::CredentialRecovery] to supply application refresh.
    pub async fn complete_invoke_model(
        &self,
        request: &CompletionRequest,
        model_id: &str,
        recovery: &dyn crate::recovery::CredentialRecovery,
    ) -> Result<CompletionResponse> {
        let body = build_anthropic_messages_body(request, model_id);
        let body_bytes = serde_json::to_vec(&body)?;
        let url = format!("{}/model/{}/invoke", self.base_url(), model_id);
        let policy = retry::RetryPolicy::default();
        let mut auth_recovered = false;

        for attempt in 1..=policy.max_attempts {
            let response = self
                .send_request("POST", &url, Some(&body_bytes), "bedrock")
                .await?;
            let status = response.status();
            // Expired bearer key: swap in a fresh one and retry exactly once.
            if auth_recover::is_auth_failure(status) && !auth_recovered && recovery.recover().await
            {
                auth_recovered = true;
                continue;
            }
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
