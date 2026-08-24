//! Provider stub unused by evidence-only recall.

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

pub(super) struct UnusedProvider;

#[async_trait]
impl Provider for UnusedProvider {
    fn name(&self) -> &str {
        "unused"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }

    async fn complete(&self, _: CompletionRequest) -> Result<CompletionResponse> {
        anyhow::bail!("provider must not run for evidence recall")
    }

    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!("provider must not run for evidence recall")
    }
}
