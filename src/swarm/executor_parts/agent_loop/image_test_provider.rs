//! Offline provider that rejects accidental completion calls in tool tests.

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

pub(super) struct OfflineProvider;

#[async_trait]
impl Provider for OfflineProvider {
    fn name(&self) -> &str {
        "offline"
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }
    async fn complete(&self, _: CompletionRequest) -> Result<CompletionResponse> {
        panic!("tool recording must not call a provider")
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        panic!("tool recording must not call a provider")
    }
}
