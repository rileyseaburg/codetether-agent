//! Mock providers for prompt event tests.

use crate::provider::*;
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::{self, BoxStream};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct StreamContextErrorProvider {
    calls: AtomicUsize,
}

impl StreamContextErrorProvider {
    pub const fn new() -> Self {
        Self {
            calls: AtomicUsize::new(0),
        }
    }

    pub fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}

#[async_trait]
impl Provider for StreamContextErrorProvider {
    fn name(&self) -> &str {
        "mock"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }

    async fn complete(&self, _: CompletionRequest) -> Result<CompletionResponse> {
        Ok(response())
    }

    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        if self.calls.fetch_add(1, Ordering::SeqCst) == 0 {
            Err(anyhow::anyhow!("Your input exceeds the context window"))
        } else {
            Ok(Box::pin(stream::iter([StreamChunk::Text("ok".into())])))
        }
    }
}

fn response() -> CompletionResponse {
    CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text { text: "ok".into() }],
        },
        usage: Usage::default(),
        finish_reason: FinishReason::Stop,
    }
}
