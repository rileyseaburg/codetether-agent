//! Non-streaming Converse completion with retry handling.

use crate::{BedrockClient, recovery as auth_recover, retry};
use anyhow::{Context, Result};
use codetether_bedrock_protocol::response::parse_converse_response;
use codetether_provider_core::{CompletionRequest, CompletionResponse};

#[path = "converse_error.rs"]
mod converse_error;

impl BedrockClient<'_> {
    /// Send a Converse request, retrying transient failures per policy.
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
    pub async fn complete_converse(
        &self,
        request: &CompletionRequest,
        model_id: &str,
        recovery: &dyn crate::recovery::CredentialRecovery,
    ) -> Result<CompletionResponse> {
        let body = codetether_bedrock_protocol::build_converse_body(request, model_id);
        // Encode model suffixes like `:0` before dispatch; the signer preserves
        // already encoded path segments instead of encoding them twice.
        let url = self.runtime_model_url(model_id, "converse");
        let body_bytes = serde_json::to_vec(&body)?;
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
                .context("Failed to read Bedrock response")?;

            if status.is_success() {
                return parse_converse_response(&text);
            }
            if retry::should_retry_status(status.as_u16()) && attempt < policy.max_attempts {
                tokio::time::sleep(policy.delay_for(attempt)).await;
                continue;
            }
            return Err(converse_error::map(status, &text));
        }
        unreachable!("retry loop exits via return");
    }
}
