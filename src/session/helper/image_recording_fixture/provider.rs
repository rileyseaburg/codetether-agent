//! Provider sentinel: these tool-only tests must never request a completion.

use crate::provider::*;
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub(in crate::session::helper) struct RejectProvider(pub AtomicUsize);

#[async_trait]
impl Provider for RejectProvider {
    fn name(&self) -> &str {
        "image-recording-mock"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(Vec::new())
    }

    async fn complete(&self, _: CompletionRequest) -> Result<CompletionResponse> {
        self.0.fetch_add(1, Ordering::SeqCst);
        anyhow::bail!("completion forbidden in mock image recording tests")
    }

    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        self.0.fetch_add(1, Ordering::SeqCst);
        anyhow::bail!("streaming forbidden in mock image recording tests")
    }
}
