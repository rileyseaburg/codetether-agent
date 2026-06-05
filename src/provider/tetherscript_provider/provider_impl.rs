//! [`Provider`] trait impl — delegates to focused sub-modules.

use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

use super::runner::TetherScriptProvider;
use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};

#[async_trait]
impl Provider for TetherScriptProvider {
    fn name(&self) -> &str {
        self.name_str()
    }

    fn supports_structured_streaming(&self) -> bool {
        false
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.available_models().await
    }

    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse> {
        self.complete_non_streaming(req).await
    }

    async fn complete_stream(
        &self,
        _req: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!(
            "Provider '{}' does not support structured streaming",
            self.name()
        )
    }
}
