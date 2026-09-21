//! Provider trait dispatch for the standalone Bonsai runtime.
use super::BonsaiProvider;
use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;
#[async_trait]
impl Provider for BonsaiProvider {
    fn name(&self) -> &str {
        "bonsai"
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![ModelInfo {
            id: super::MODEL.into(),
            name: "Ternary Bonsai 2 27B PQ2".into(),
            provider: "bonsai".into(),
            context_window: 4096,
            max_output_tokens: Some(1024),
            supports_vision: false,
            supports_tools: false,
            supports_streaming: true,
            input_cost_per_million: Some(0.0),
            output_cost_per_million: Some(0.0),
        }])
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        super::completion::collect(super::stream::start(self, request)?).await
    }
    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        super::stream::start(self, request)
    }
}
