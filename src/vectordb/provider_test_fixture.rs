//! A deterministic embedding provider fixture for vector DB tests.

use crate::provider::{
    CompletionRequest, CompletionResponse, EmbeddingRequest, EmbeddingResponse, ModelInfo,
    Provider, StreamChunk, Usage,
};
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

/// Returns a fixed `[3.0, 4.0]` vector per input (normalizes to length 1).
pub(super) struct FixedEmbedProvider;

#[async_trait]
impl Provider for FixedEmbedProvider {
    fn name(&self) -> &str {
        "fixed"
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![])
    }
    async fn complete(&self, _: CompletionRequest) -> Result<CompletionResponse> {
        anyhow::bail!("unused")
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!("unused")
    }
    async fn embed(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        let embeddings = request.inputs.iter().map(|_| vec![3.0, 4.0]).collect();
        Ok(EmbeddingResponse {
            embeddings,
            usage: Usage::default(),
        })
    }
}
