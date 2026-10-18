//! Scripted provider for completed-output checkpoint recovery tests.

use std::collections::VecDeque;
use std::sync::Mutex;

use anyhow::Result;
use futures::stream::{self, BoxStream};

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};

pub(in crate::session::helper::stream) struct CheckpointProvider {
    pub(in crate::session::helper::stream) streams: Mutex<VecDeque<Vec<StreamChunk>>>,
    pub(in crate::session::helper::stream) requests: Mutex<Vec<CompletionRequest>>,
}

#[async_trait::async_trait]
impl Provider for CheckpointProvider {
    fn name(&self) -> &str {
        "checkpoint-mock"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }

    async fn complete(&self, _request: CompletionRequest) -> Result<CompletionResponse> {
        anyhow::bail!("unused")
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        self.requests.lock().unwrap().push(request);
        let chunks = self.streams.lock().unwrap().pop_front().unwrap_or_default();
        Ok(Box::pin(stream::iter(chunks)))
    }
}
