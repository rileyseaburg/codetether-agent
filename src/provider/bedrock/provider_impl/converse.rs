//! Non-streaming Converse completion with retry handling.

use crate::provider::bedrock::response::parse_converse_response;
use crate::provider::bedrock::{BedrockProvider, auth_recover, retry};
use crate::provider::{CompletionRequest, CompletionResponse};
use anyhow::{Context, Result};

#[path = "converse_error.rs"]
mod converse_error;

impl BedrockProvider {
    /// Send a Converse request, retrying transient failures per policy.
    pub(in crate::provider::bedrock) async fn complete_converse(
        &self,
        request: &CompletionRequest,
        model_id: &str,
    ) -> Result<CompletionResponse> {
        let body = self.build_converse_body(request, model_id);
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
            if auth_recover::is_auth_failure(status)
                && !auth_recovered
                && auth_recover::recover(self).await
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
