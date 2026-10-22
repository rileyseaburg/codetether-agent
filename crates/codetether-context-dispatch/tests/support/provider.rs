use async_trait::async_trait;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk,
};

pub struct NoCalls;
#[async_trait]
impl Provider for NoCalls {
    fn name(&self) -> &str {
        "fixture"
    }
    async fn list_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        Ok(vec![])
    }
    async fn complete(&self, _: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        anyhow::bail!("this deterministic derivation must not call a model")
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> anyhow::Result<futures::stream::BoxStream<'static, StreamChunk>> {
        anyhow::bail!("this deterministic derivation must not call a model")
    }
}
