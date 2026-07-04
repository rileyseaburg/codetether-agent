//! Mock provider for the SRP restart-engine tests ([`super::restart_tests`]).

use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Result;
use futures::stream::{self, BoxStream};

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};

/// Yields a partial-then-EOF (no `Done`) on the first `complete_stream` call,
/// then a complete `Text` + `Done` stream on every subsequent call.
pub(super) struct FlakyThenCompleteProvider {
    pub(super) calls: AtomicUsize,
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
        let n = self.calls.fetch_add(1, Ordering::SeqCst);
        if n == 0 {
            Ok(Box::pin(stream::iter(vec![StreamChunk::Text(
                "truncated".into(),
            )])))
        } else {
            Ok(Box::pin(stream::iter(vec![
                StreamChunk::Text("complete answer".into()),
                StreamChunk::Done { usage: None },
            ])))
        }
    }
}
