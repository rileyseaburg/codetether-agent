//! An inert provider: scheduling tests must never call a live model.
use codetether_provider_core::*;
pub(super) struct MockProvider;
#[async_trait::async_trait]
impl Provider for MockProvider {
    fn name(&self) -> &str {
        "mock"
    }
    async fn list_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }
    async fn complete(&self, _: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        anyhow::bail!("live completion must not run")
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> anyhow::Result<futures::stream::BoxStream<'static, StreamChunk>> {
        anyhow::bail!("live streaming must not run")
    }
}
pub(super) fn selected() -> codetether_swarm_tool::runtime::SelectedProvider {
    codetether_swarm_tool::runtime::SelectedProvider {
        provider: std::sync::Arc::new(MockProvider),
        model: codetether_swarm_tool::model_selection::resolve(Some("mock/test"), &["mock"])
            .unwrap(),
    }
}
