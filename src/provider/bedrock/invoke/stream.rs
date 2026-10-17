//! Delegate Bedrock execution to the client crate.
use crate::provider::bedrock::BedrockProvider;
use anyhow::Result;
use codetether_provider_core::{CompletionRequest, StreamChunk};
impl BedrockProvider {
    pub(in crate::provider::bedrock) async fn invoke_model_stream_adapter(
        &self,
        request: &CompletionRequest,
        model_id: &str,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        self.bedrock_client()
            .invoke_model_stream_adapter(request, model_id, self)
            .await
    }
}
