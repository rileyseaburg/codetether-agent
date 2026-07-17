//! Mock provider that stalls after partial output on its first request.

use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Result;
use futures::StreamExt;
use futures::stream::{self, BoxStream};

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};

pub(super) struct StallThenCompleteProvider {
    pub(super) calls: AtomicUsize,
}

#[async_trait::async_trait]
impl Provider for StallThenCompleteProvider {
    fn name(&self) -> &str {
        "stall-mock"
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
            let partial = stream::iter([StreamChunk::Text("truncated".into())]);
            return Ok(Box::pin(partial.chain(stream::pending())));
        }
        Ok(Box::pin(stream::iter([
            StreamChunk::Text("complete answer".into()),
            StreamChunk::Done { usage: None },
        ])))
    }
}
