//! Scripted provider fixture for reviewer tests (no network).

use std::sync::Arc;

use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::config::{ReviewConfig, ReviewMode};
use crate::provider::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    ProviderRegistry, Role, StreamChunk, Usage,
};

/// Replies with a fixed final message on every turn.
pub(super) struct Scripted(pub(super) &'static str);

#[async_trait]
impl Provider for Scripted {
    fn name(&self) -> &str {
        "scripted"
    }
    async fn list_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }
    async fn complete(&self, _: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text {
                    text: self.0.to_string(),
                }],
            },
            usage: Usage::default(),
            finish_reason: FinishReason::Stop,
        })
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> anyhow::Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!("unused")
    }
}

pub(super) fn registry(reply: &'static str) -> ProviderRegistry {
    let mut registry = ProviderRegistry::new();
    registry.register(Arc::new(Scripted(reply)));
    registry
}

pub(super) fn config() -> ReviewConfig {
    ReviewConfig {
        mode: ReviewMode::Advise,
        model: Some("scripted/any".into()),
        max_steps: Some(3),
        timeout_secs: Some(10),
    }
}
