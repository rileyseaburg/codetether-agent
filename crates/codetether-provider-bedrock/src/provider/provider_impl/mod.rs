//! `Provider` trait implementation for [`BedrockProvider`].
//!
//! The trait methods stay thin: they resolve the model id, validate auth,
//! and dispatch to native InvokeModel or Converse. Fable 5.2 streaming
//! uses Converse so callers receive deltas without buffering the full answer;
//! non-streaming and older Fable requests retain their native adapter.

mod converse;
mod runtime_url;
#[cfg(test)]
mod tests;

use crate::provider::BedrockProvider;
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk,
};

#[async_trait]
impl Provider for BedrockProvider {
    fn name(&self) -> &str {
        "bedrock"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.prepare_auth().await?;
        self.discover_models().await
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let model_id = Self::resolve_model_id(&request.model);
        self.prepare_auth().await?;
        if Self::should_use_invoke_model(model_id) {
            return self.complete_invoke_model(&request, model_id).await;
        }
        self.complete_converse(&request, model_id).await
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let model_id = Self::resolve_model_id(&request.model);
        self.prepare_auth().await?;
        if Self::should_use_invoke_model_stream(model_id) {
            return self.invoke_model_stream_adapter(&request, model_id).await;
        }
        let body = self.build_converse_body(&request, model_id);
        let body_bytes = serde_json::to_vec(&body)?;
        self.converse_stream(model_id, body_bytes).await
    }
}
