//! `Provider` trait implementation for [`BedrockProvider`].
//!
//! The trait methods stay thin: they resolve the model id, validate auth,
//! route `claude-fable-*` models to the InvokeModel adapter, and otherwise
//! delegate to the Converse helpers in [`converse`].

mod converse;

use crate::provider::bedrock::BedrockProvider;
use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
impl Provider for BedrockProvider {
    fn name(&self) -> &str {
        "bedrock"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.validate_auth()?;
        self.discover_models().await
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let model_id = Self::resolve_model_id(&request.model);
        self.validate_auth()?;
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
        self.validate_auth()?;
        if Self::should_use_invoke_model(model_id) {
            return self.invoke_model_stream_adapter(&request, model_id).await;
        }
        let body = self.build_converse_body(&request, model_id);
        let body_bytes = serde_json::to_vec(&body)?;
        self.converse_stream(model_id, body_bytes).await
    }
}
