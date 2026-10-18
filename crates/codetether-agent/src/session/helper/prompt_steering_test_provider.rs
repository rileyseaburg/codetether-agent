//! Provider that injects steering during its first streamed response.

use crate::provider::*;
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::{self, BoxStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::prompt_steering_test_support::{has_text, response};

pub struct SteeringProvider {
    session_id: String,
    calls: AtomicUsize,
    saw_steering: AtomicBool,
}

impl SteeringProvider {
    pub fn new(session_id: String) -> Self {
        Self { session_id, calls: AtomicUsize::new(0), saw_steering: AtomicBool::new(false) }
    }

    pub fn outcome(&self) -> (usize, bool) {
        (self.calls.load(Ordering::SeqCst), self.saw_steering.load(Ordering::SeqCst))
    }
}

#[async_trait]
impl Provider for SteeringProvider {
    fn name(&self) -> &str { "mock" }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> { Ok(Vec::new()) }

    async fn complete(&self, _: CompletionRequest) -> Result<CompletionResponse> {
        Ok(response("fallback"))
    }

    async fn complete_stream(&self, request: CompletionRequest) -> Result<BoxStream<'static, StreamChunk>> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst);
        if call == 0 {
            let input = crate::session::helper::steering::SteeringInput::new("adjust".into(), vec![]);
            assert!(crate::session::helper::steering::push(&self.session_id, input));
        } else {
            self.saw_steering.store(has_text(&request, "adjust"), Ordering::SeqCst);
        }
        let text = if call == 0 { "first" } else { "final" };
        Ok(Box::pin(stream::iter([StreamChunk::Text(text.into()), StreamChunk::Done { usage: None }])))
    }
}
