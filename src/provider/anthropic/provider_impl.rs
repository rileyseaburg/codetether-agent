//! Provider trait implementation for Anthropic-compatible backends.
//!
//! This module connects [`AnthropicProvider`] to the application's generic
//! [`Provider`] interface. It exposes provider identity, static model metadata,
//! non-streaming completion execution, and the explicit structured-streaming
//! capability decision used by provider routing.

use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};

use super::AnthropicProvider;

/// Implements the generic provider contract for Anthropic-compatible services.
///
/// The implementation uses the configured provider name as the public provider
/// identifier, validates that an API key is available before returning model
/// metadata, delegates non-streaming completions to
/// [`AnthropicProvider::complete_non_streaming`], and rejects structured
/// streaming requests because this backend path does not expose a compatible
/// stream adapter.
#[async_trait]
impl Provider for AnthropicProvider {
    /// Return the configured provider name used for routing and diagnostics.
    ///
    /// # Returns
    ///
    /// A borrowed provider identifier such as `anthropic`, `minimax`, or another
    /// Anthropic-compatible backend name.
    fn name(&self) -> &str {
        &self.provider_name
    }

    /// Report whether this provider supports the structured streaming API.
    ///
    /// # Returns
    ///
    /// Always `false`; callers must use [`Provider::complete`] for this
    /// implementation.
    fn supports_structured_streaming(&self) -> bool {
        false
    }

    /// List static models available for this configured backend.
    ///
    /// The API key is validated before returning models so an unconfigured
    /// provider fails early even though the catalog itself is local metadata.
    ///
    /// # Returns
    ///
    /// A vector of [`ModelInfo`] entries for the selected provider catalog.
    ///
    /// # Errors
    ///
    /// Returns an error when the provider API key is missing or invalid
    /// according to [`AnthropicProvider::validate_api_key`].
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.validate_api_key()?;
        Ok(self.available_models())
    }

    /// Execute a non-streaming completion request.
    ///
    /// # Parameters
    ///
    /// * `request` - Provider-neutral completion request containing model,
    ///   messages, tools, and generation options.
    ///
    /// # Returns
    ///
    /// A normalized [`CompletionResponse`] parsed from the
    /// Anthropic-compatible Messages API response.
    ///
    /// # Errors
    ///
    /// Returns an error if request conversion, HTTP execution, response
    /// deserialization, or provider error handling fails.
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.complete_non_streaming(request).await
    }

    /// Reject structured streaming requests for this provider implementation.
    ///
    /// # Parameters
    ///
    /// * `_request` - Completion request that would be streamed if this
    ///   provider supported structured streaming.
    ///
    /// # Returns
    ///
    /// This method always returns an error and never yields a stream.
    ///
    /// # Errors
    ///
    /// Always returns an error explaining that the configured provider does not
    /// support structured streaming through this implementation.
    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!(
            "Provider '{}' does not support structured streaming",
            self.name()
        )
    }
}
