//! Provider that succeeds only after transport fallback.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use anyhow::Result;
use futures::stream::{self, BoxStream};

use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk,
};

pub(in crate::runtime_tests) struct FallbackProvider {
    pub(in crate::runtime_tests) calls: AtomicUsize,
    pub(in crate::runtime_tests) begins: AtomicUsize,
    pub(in crate::runtime_tests) fallback: AtomicBool,
    pub(in crate::runtime_tests) allow_fallback: bool,
}

#[async_trait::async_trait]
impl Provider for FallbackProvider {
    fn name(&self) -> &str {
        "fallback-mock"
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
        self.calls.fetch_add(1, Ordering::SeqCst);
        let chunks = if self.fallback.load(Ordering::SeqCst) {
            vec![
                StreamChunk::Text("http".into()),
                StreamChunk::Done { usage: None },
            ]
        } else {
            vec![StreamChunk::Error("websocket connection reset".into())]
        };
        Ok(Box::pin(stream::iter(chunks)))
    }

    fn begin_stream_recovery(&self, _session_id: &str) {
        self.begins.fetch_add(1, Ordering::SeqCst);
    }

    fn try_stream_fallback(&self, _request: &CompletionRequest, _session_id: &str) -> bool {
        self.allow_fallback && !self.fallback.swap(true, Ordering::SeqCst)
    }
}
