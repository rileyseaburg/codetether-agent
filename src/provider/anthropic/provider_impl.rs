//! Provider trait implementation for Anthropic-compatible backends.
//!
//! This module connects [`AnthropicProvider`] to the application's generic
//! [`Provider`] interface. It exposes provider identity, static model metadata,
//! streaming and non-streaming completion, and the structured-streaming
//! capability flag used by provider routing.

use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};

use super::AnthropicProvider;

/// Implements the generic provider contract for Anthropic-compatible services.
#[async_trait]
impl Provider for AnthropicProvider {
    /// Return the configured provider name used for routing and diagnostics.
    fn name(&self) -> &str {
        &self.provider_name
    }

    /// Report whether this provider supports the structured streaming API.
    ///
    /// Always `true`; streaming is implemented via SSE event parsing.
    fn supports_structured_streaming(&self) -> bool {
        true
    }

    /// List static models available for this configured backend.
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.validate_api_key()?;
        Ok(self.available_models())
    }

    /// Execute a non-streaming completion request.
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.complete_non_streaming(request).await
    }

    /// Execute a streaming completion request via SSE.
    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        self.validate_api_key()?;
        let body = super::body::build_streaming(&request, self.enable_prompt_caching);
        super::complete_stream::start(&self.client, &self.base_url, &self.api_key, &body).await
    }
}
