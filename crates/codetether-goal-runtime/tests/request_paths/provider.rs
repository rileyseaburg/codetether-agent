use codetether_provider_core::*;
pub struct NeverCalled;
#[async_trait::async_trait]
impl Provider for NeverCalled {
    fn name(&self) -> &str {
        "fixture-no-network"
    }
    async fn list_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        Ok(vec![])
    }
    async fn complete(&self, _: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        anyhow::bail!("network must not be called")
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> anyhow::Result<futures::stream::BoxStream<'static, StreamChunk>> {
        anyhow::bail!("network must not be called")
    }
}
