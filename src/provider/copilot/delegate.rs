//! Delegate provider operations and image-test conversion to the protocol crate.
use super::CopilotProvider;
use crate::provider::{
    CompletionRequest, CompletionResponse, Message, ModelInfo, Provider, StreamChunk,
};
use anyhow::Result;
use async_trait::async_trait;
#[async_trait]
impl Provider for CopilotProvider {
    fn name(&self) -> &str {
        self.0.name()
    }
    fn supports_structured_streaming(&self) -> bool {
        self.0.supports_structured_streaming()
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.0.list_models().await
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.0.complete(request).await
    }
    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        self.0.complete_stream(request).await
    }
}
impl CopilotProvider {
    pub(in crate::provider) fn convert_messages(messages: &[Message]) -> Vec<serde_json::Value> {
        codetether_provider_copilot::CopilotProvider::convert_messages(messages)
    }
}
