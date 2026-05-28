use crate::provider::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, Usage,
};
use anyhow::Result;
use futures::stream::{self, BoxStream};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
pub(super) struct ContextErrorUntilCompactProvider {
    calls: AtomicUsize,
    saw_compact_schema: AtomicBool,
}
impl ContextErrorUntilCompactProvider {
    pub(super) fn new() -> Self {
        Self {
            calls: AtomicUsize::new(0),
            saw_compact_schema: AtomicBool::new(false),
        }
    }
    pub(super) fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
    pub(super) fn saw_compact_schema(&self) -> bool {
        self.saw_compact_schema.load(Ordering::SeqCst)
    }
}
#[async_trait::async_trait]
impl Provider for ContextErrorUntilCompactProvider {
    fn name(&self) -> &str {
        "mock"
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let compact = request.tools.iter().all(|tool| {
            tool.parameters.get("additionalProperties") == Some(&serde_json::Value::Bool(true))
        });
        if compact {
            self.saw_compact_schema.store(true, Ordering::SeqCst);
            return Ok(CompletionResponse {
                message: Message {
                    role: Role::Assistant,
                    content: vec![ContentPart::Text { text: "ok".into() }],
                },
                usage: Usage::default(),
                finish_reason: FinishReason::Stop,
            });
        }
        anyhow::bail!(
            "Your input exceeds the context window of this model. Please adjust your input and try again."
        )
    }
    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        Ok(Box::pin(stream::empty()))
    }
}
pub(super) fn provider_arc() -> Arc<ContextErrorUntilCompactProvider> {
    Arc::new(ContextErrorUntilCompactProvider::new())
}
