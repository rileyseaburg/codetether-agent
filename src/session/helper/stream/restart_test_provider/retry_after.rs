use anyhow::Result;
use futures::stream::{self, BoxStream};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};

pub(in crate::session::helper::stream) struct RetryAfterProvider {
    pub(in crate::session::helper::stream) calls: AtomicUsize,
}

#[async_trait::async_trait]
impl Provider for RetryAfterProvider {
    fn name(&self) -> &str {
        "retry-after-mock"
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
        let chunks = if self.calls.fetch_add(1, Ordering::SeqCst) == 0 {
            vec![StreamChunk::Error(
                "service unavailable; try again in 35 seconds".into(),
            )]
        } else {
            vec![
                StreamChunk::Text("recovered".into()),
                StreamChunk::Done { usage: None },
            ]
        };
        Ok(Box::pin(stream::iter(chunks)))
    }
}
