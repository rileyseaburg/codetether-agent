//! Mock provider verifies that side answers cannot invoke tools.

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use anyhow::Result;
use async_trait::async_trait;

pub(super) struct ReplyProvider(pub bool);

#[async_trait]
impl Provider for ReplyProvider {
    fn name(&self) -> &str {
        "answer-test"
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        assert!(request.tools.is_empty());
        if self.0 {
            anyhow::bail!("fixture provider error");
        }
        Ok(super::reply::response())
    }
    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        Ok(Box::pin(futures::stream::empty()))
    }
}
