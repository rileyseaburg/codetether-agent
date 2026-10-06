//! Captures requests at the actual provider boundary.
use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use anyhow::Result;
use futures::stream::BoxStream;
#[path = "mock/capture.rs"]
mod capture;
pub(super) use capture::Capture;

#[async_trait::async_trait]
impl Provider for Capture {
    fn name(&self) -> &str {
        "local_cuda"
    }
    fn resolved_model_identity(&self, model: &str) -> String {
        model.strip_prefix("alias:").unwrap_or(model).to_owned()
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.record(request, None);
        anyhow::bail!("capture only")
    }
    async fn complete_scoped(
        &self,
        request: CompletionRequest,
        session: &str,
    ) -> Result<CompletionResponse> {
        self.record(request, Some(session));
        anyhow::bail!("capture only")
    }
    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        self.record(request, None);
        Ok(Box::pin(futures::stream::empty()))
    }
    async fn complete_stream_scoped(
        &self,
        request: CompletionRequest,
        session: &str,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        self.record(request, Some(session));
        Ok(Box::pin(futures::stream::empty()))
    }
}
