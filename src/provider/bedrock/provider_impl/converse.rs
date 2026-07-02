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
        // Keep the runtime URL readable; the SigV4 signer canonicalizes path
        // segments so model suffixes like `:0` are encoded exactly once.
        let url = format!("{}/model/{}/converse", self.base_url(), model_id);
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
