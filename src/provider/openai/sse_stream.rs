//! Raw-SSE streaming for reasoning-capable OpenAI-compatible providers.
//!
//! Bypasses the typed `async-openai` stream (which drops `reasoning_content`)
//! by parsing `data:` frames directly. Used for providers like Cerebras whose
//! reasoning models can emit reasoning-only turns.

use anyhow::{Context, Result};
use futures::StreamExt;
use serde_json::Value;

use crate::provider::{CompletionRequest, StreamChunk};

use super::OpenAIProvider;
use super::sse_drive::drive;

impl OpenAIProvider {
    /// True for providers whose models stream vendor `reasoning_content`.
    pub(super) fn uses_reasoning_stream(&self) -> bool {
        self.provider_name == "cerebras"
    }

    /// Stream a completion via raw SSE, capturing reasoning content.
    pub(super) async fn reasoning_stream(
        &self,
        request: CompletionRequest,
        body: Value,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let url = format!("{}/chat/completions", self.api_base.trim_end_matches('/'));
        let mut req = self.http.post(url).json(&body);
        if let Some(key) = self.api_key.as_deref().filter(|k| !k.is_empty()) {
            req = req.bearer_auth(key);
        }
        let response = req.send().await.context("send reasoning stream request")?;
        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            let msg = format!("{} stream error ({status}): {}", request.model, text);
            return Ok(futures::stream::iter(vec![
                StreamChunk::Error(msg),
                StreamChunk::Done { usage: None },
            ])
            .boxed());
        }
        Ok(drive(response.bytes_stream()).boxed())
    }
}
