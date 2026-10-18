//! Delegate provider operations and image-test conversion to the protocol crate.
use super::{CopilotProvider, Identity};
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, Message, ModelInfo, Provider, StreamChunk,
};
#[async_trait]
impl<I: Identity> Provider for CopilotProvider<I> {
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
impl<I: Identity> CopilotProvider<I> {
    /// Convert provider-neutral messages using the shared Copilot wire format.
    /// # Arguments
    /// * `messages` — Ordered conversation messages to encode.
    /// # Returns
    /// JSON message records used by the underlying Copilot provider.
    /// # Examples
    /// See [`crate::CopilotProvider::convert_messages`] for the wire conversion.
    pub fn convert_messages(messages: &[Message]) -> Vec<serde_json::Value> {
        crate::CopilotProvider::convert_messages(messages)
    }
}
