//! Provider trait implementation for DeepSeek.

use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};

use super::DeepSeekProvider;
use super::models;

#[async_trait]
impl Provider for DeepSeekProvider {
    fn name(&self) -> &str {
        "deepseek"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(models::list())
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        super::complete::exec(self, request).await
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        super::stream::exec(self, request).await
    }
}
