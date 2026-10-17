//! Behavioral defaults of the shared provider contract.

use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Probe {
    calls: AtomicUsize,
}
#[async_trait::async_trait]
impl Provider for Probe {
    fn name(&self) -> &str {
        "bedrock"
    }
    async fn list_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }
    async fn complete(&self, _request: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        anyhow::bail!("completion reached")
    }
    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> anyhow::Result<futures::stream::BoxStream<'static, StreamChunk>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(Box::pin(futures::stream::empty()))
    }
}
fn request() -> CompletionRequest {
    CompletionRequest {
        model: "nova-pro".into(),
        messages: Vec::new(),
        tools: Vec::new(),
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: Vec::new(),
    }
}

mod defaults;
