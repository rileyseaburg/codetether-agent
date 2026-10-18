//! Delegate Bedrock execution to the client crate.
use crate::provider::BedrockProvider;
use anyhow::Result;
use codetether_provider_core::{CompletionRequest, CompletionResponse};
impl BedrockProvider {
    /// Send a Converse request, retrying transient failures per policy.
    pub(in crate::provider) async fn complete_converse(
        &self,
        request: &CompletionRequest,
        model_id: &str,
    ) -> Result<CompletionResponse> {
        self.bedrock_client()
            .complete_converse(request, model_id, self)
            .await
    }
}
