//! Mock provider implementation for tests.

use crate::provider::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, Provider, Role,
    StreamChunk, Usage,
};
use anyhow::Result;
use futures::stream::{self, BoxStream};

use super::test_support::ContextErrorUntilCompactProvider;

#[async_trait::async_trait]
impl Provider for ContextErrorUntilCompactProvider {
    fn name(&self) -> &str {
        "mock"
    }
    async fn list_models(&self) -> Result<Vec<crate::provider::ModelInfo>> {
        Ok(Vec::new())
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let compact = request.tools.iter().all(|tool| {
            tool.parameters.get("additionalProperties") == Some(&serde_json::Value::Bool(true))
        });
        if compact {
            self.saw_compact_schema
                .store(true, std::sync::atomic::Ordering::SeqCst);
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
