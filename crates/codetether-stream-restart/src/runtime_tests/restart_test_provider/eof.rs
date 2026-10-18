//! Provider that closes once before emitting a complete response.

use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Result;
use futures::stream::{self, BoxStream};

use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk,
};

pub(in crate::runtime_tests) struct FlakyThenCompleteProvider {
    pub(in crate::runtime_tests) calls: AtomicUsize,
}

#[async_trait::async_trait]
impl Provider for FlakyThenCompleteProvider {
    fn name(&self) -> &str {
        "flaky-mock"
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
        if self.calls.fetch_add(1, Ordering::SeqCst) == 0 {
            return Ok(Box::pin(stream::iter([StreamChunk::Text(
                "truncated".into(),
            )])));
        }
        Ok(Box::pin(stream::iter([
            StreamChunk::Text("complete answer".into()),
            StreamChunk::Done { usage: None },
        ])))
    }
}
