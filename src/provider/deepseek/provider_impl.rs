//! Provider trait implementation for DeepSeek.

use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};

use super::DeepSeekProvider;

#[async_trait]
impl Provider for DeepSeekProvider {
    fn name(&self) -> &str {
        "deepseek"
    }

    fn supports_structured_streaming(&self) -> bool {
        false
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(self.available_models())
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.complete_non_streaming(request).await
    }

    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!(
            "Provider '{}' does not support structured streaming",
            self.name()
        )
    }
}
