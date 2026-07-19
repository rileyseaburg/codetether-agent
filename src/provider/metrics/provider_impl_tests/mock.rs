use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use anyhow::Result;
use futures::stream::BoxStream;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub(super) struct RecoveryProvider {
    pub(super) begins: AtomicUsize,
    pub(super) fallbacks: AtomicUsize,
}

#[async_trait::async_trait]
impl Provider for RecoveryProvider {
    fn name(&self) -> &str {
        "recovery-mock"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }

    async fn complete(&self, _request: CompletionRequest) -> Result<CompletionResponse> {
        anyhow::bail!("unused")
    }

    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!("unused")
    }

    fn begin_stream_recovery(&self, _session_id: &str) {
        self.begins.fetch_add(1, Ordering::SeqCst);
    }

    fn try_stream_fallback(&self, _request: &CompletionRequest, _session_id: &str) -> bool {
        self.fallbacks.fetch_add(1, Ordering::SeqCst);
        true
    }
}
