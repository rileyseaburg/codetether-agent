//! Non-streaming Converse completion with retry handling.

use crate::provider::bedrock::response::parse_converse_response;
use crate::provider::bedrock::{BedrockError, BedrockProvider, retry};
use crate::provider::{CompletionRequest, CompletionResponse};
use crate::util;
use anyhow::{Context, Result};

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

        for attempt in 1..=policy.max_attempts {
            let response = self
                .send_request("POST", &url, Some(&body_bytes), "bedrock")
                .await?;
            let status = response.status();
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
            if let Ok(err) = serde_json::from_str::<BedrockError>(&text) {
                anyhow::bail!("Bedrock API error ({}): {}", status, err.message);
            }
            anyhow::bail!(
                "Bedrock API error: {} {}",
                status,
                util::truncate_bytes_safe(&text, 500)
            );
        }
        unreachable!("retry loop exits via return or bail!");
    }
}
