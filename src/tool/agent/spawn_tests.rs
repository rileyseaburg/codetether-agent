use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

#[path = "spawn_persistence_tests.rs"]
mod persistence;

struct MockProvider;

#[async_trait]
impl Provider for MockProvider {
    fn name(&self) -> &str {
        "mock"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![ModelInfo {
            id: "paid".into(),
            name: "paid".into(),
            provider: "mock".into(),
            context_window: 1,
            max_output_tokens: None,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: false,
            input_cost_per_million: Some(1.0),
            output_cost_per_million: Some(1.0),
        }])
    }

    async fn complete(&self, _: CompletionRequest) -> Result<CompletionResponse> {
        anyhow::bail!("unused")
    }

    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!("unused")
    }
}
