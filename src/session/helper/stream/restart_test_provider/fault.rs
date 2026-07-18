//! Provider that reports one retryable error after partial text.

use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Result;
use futures::stream::{self, BoxStream};

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};

pub(in crate::session::helper::stream) struct TransientFaultThenCompleteProvider {
    pub(in crate::session::helper::stream) calls: AtomicUsize,
}

#[async_trait::async_trait]
impl Provider for TransientFaultThenCompleteProvider {
    fn name(&self) -> &str {
        "transient-fault-mock"
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
            return Ok(Box::pin(stream::iter([
                StreamChunk::Text("discarded partial".into()),
                StreamChunk::Error("processing your request; you can retry".into()),
            ])));
        }
        Ok(Box::pin(stream::iter([
            StreamChunk::Text("complete answer".into()),
            StreamChunk::Done { usage: None },
        ])))
    }
}
