//! Minimal provider implementation for shared worker control tests.

use super::Mock;
use crate::provider::*;
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

#[async_trait]
impl Provider for Mock {
    fn name(&self) -> &str {
        "mock"
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![])
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.respond(request).await
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!("worker tests use complete")
    }
}
