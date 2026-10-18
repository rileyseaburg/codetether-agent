//! Delegate Bedrock execution to the client crate.
use crate::provider::BedrockProvider;
use anyhow::Result;
use codetether_provider_core::{CompletionRequest, CompletionResponse};
impl BedrockProvider {
    /// Complete a request via the Bedrock InvokeModel API.
    ///
    /// Retries transient throttling/5xx failures with exponential backoff +
    /// jitter, matching the Converse path's [`crate::provider::retry::RetryPolicy`].
    pub(in crate::provider) async fn complete_invoke_model(
        &self,
        request: &CompletionRequest,
        model_id: &str,
    ) -> Result<CompletionResponse> {
        self.bedrock_client()
            .complete_invoke_model(request, model_id, self)
            .await
    }
}
