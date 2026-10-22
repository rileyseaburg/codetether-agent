use async_trait::async_trait;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk,
};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub struct RejectingProvider(pub AtomicUsize);
#[async_trait]
impl Provider for RejectingProvider {
    fn name(&self) -> &str {
        "offline-state-evaluation"
    }
    async fn list_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        Ok(vec![])
    }
    async fn complete(&self, _: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        self.0.fetch_add(1, Ordering::SeqCst);
        anyhow::bail!("injected provider outage; no network request was sent")
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> anyhow::Result<futures::stream::BoxStream<'static, StreamChunk>> {
        self.0.fetch_add(1, Ordering::SeqCst);
        anyhow::bail!("injected provider outage; no network request was sent")
    }
}

pub fn hooks() -> codetether_context_legacy::Hooks {
    codetether_context_legacy::Hooks {
        apply: |_| Default::default(),
        repair: |_| Default::default(),
        resolver: |provider, model, _, _| {
            let model = model.to_owned();
            Box::pin(async move { (provider, model) })
        },
    }
}
