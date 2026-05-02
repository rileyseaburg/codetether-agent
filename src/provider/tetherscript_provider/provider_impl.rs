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

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.call_list_models()).await?
    }

    async fn complete(&self, req: CompletionRequest) -> Result<CompletionResponse> {
        super::complete::complete(self, req).await
    }

    async fn complete_stream(
        &self,
        req: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        super::stream::complete_stream(self, req).await
    }
}
