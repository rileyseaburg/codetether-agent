//! Count inference calls and return deterministic title responses or failures.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, Usage,
};
use futures::stream::BoxStream;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(super) struct MockProvider {
    pub name: &'static str,
    pub reply: Option<&'static str>,
    pub calls: AtomicUsize,
}

#[async_trait]
impl Provider for MockProvider {
    fn name(&self) -> &str {
        self.name
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        panic!("title generation must not discover models")
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(request.model, "selected-model");
        let Some(text) = self.reply else {
            anyhow::bail!("mock inference failure");
        };
        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text { text: text.into() }],
            },
            usage: Usage::default(),
            finish_reason: FinishReason::Stop,
        })
    }

    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        panic!("title generation must not stream")
    }
}
