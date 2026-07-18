//! The [`Provider`] trait and [`ModelInfo`] struct.
//!
//! Every AI backend (OpenAI, Anthropic, Bedrock, etc.) implements [`Provider`]
//! so the [`ProviderRegistry`](super::ProviderRegistry) can dispatch requests
//! uniformly.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::{FinishReason, ModelInfo};
//!
//! let info = ModelInfo {
//!     id: "gpt-4o".into(),
//!     name: "GPT-4o".into(),
//!     provider: "openai".into(),
//!     context_window: 128_000,
//!     max_output_tokens: Some(16_384),
//!     supports_vision: true,
//!     supports_tools: true,
//!     supports_streaming: true,
//!     input_cost_per_million: Some(2.5),
//!     output_cost_per_million: Some(10.0),
//! };
//! assert!(info.supports_vision);
//! ```

use anyhow::Result;
use async_trait::async_trait;

#[path = "traits/model_info.rs"]
mod model_info;
pub use model_info::ModelInfo;

use super::types::{
    CompletionRequest, CompletionResponse, EmbeddingRequest, EmbeddingResponse, StreamChunk,
};

/// Trait that all AI providers must implement.
#[async_trait]
pub trait Provider: Send + Sync {
    /// Provider identifier (e.g. `"openai"`, `"bedrock"`).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use codetether_agent::provider::Provider;
    /// # fn demo(p: &dyn Provider) {
    /// assert!(!p.name().is_empty());
    /// # }
    /// ```
    fn name(&self) -> &str;

    /// List models available under this provider.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use codetether_agent::provider::Provider;
    /// # async fn demo(p: &dyn Provider) {
    /// let models = p.list_models().await.unwrap();
    /// assert!(!models.is_empty());
    /// # }
    /// ```
    async fn list_models(&self) -> Result<Vec<ModelInfo>>;

    /// Generate a single completion.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use codetether_agent::provider::{Provider, CompletionRequest};
    /// # async fn demo(p: &dyn Provider, req: CompletionRequest) {
    /// let resp = p.complete(req).await.unwrap();
    /// # }
    /// ```
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;

    /// Generate a completion with transport state isolated to one session.
    ///
    /// # Arguments
    ///
    /// * `request` — Model, messages, tools, and sampling configuration.
    /// * `_session_id` — Stable conversation identifier used for isolation.
    ///
    /// # Returns
    ///
    /// The completed response, or a provider error.
    async fn complete_scoped(
        &self,
        request: CompletionRequest,
        _session_id: &str,
    ) -> Result<CompletionResponse> {
        self.complete(request).await
    }

    /// Generate a streaming completion.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use codetether_agent::provider::{Provider, CompletionRequest};
    /// # async fn demo(p: &dyn Provider, req: CompletionRequest) {
    /// let stream = p.complete_stream(req).await.unwrap();
    /// # }
    /// ```
    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>>;

    /// Stream with connection health and pooling isolated to one session.
    ///
    /// # Arguments
    ///
    /// * `request` — Model, messages, tools, and sampling configuration.
    /// * `_session_id` — Stable conversation identifier used for isolation.
    ///
    /// # Returns
    ///
    /// A stream of response chunks, or a provider error.
    async fn complete_stream_scoped(
        &self,
        request: CompletionRequest,
        _session_id: &str,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        self.complete_stream(request).await
    }

    /// Whether streaming preserves structured output such as tool calls.
    fn supports_structured_streaming(&self) -> bool {
        true
    }

    /// Generate embeddings (optional; default returns an error).
    async fn embed(&self, _request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        anyhow::bail!("Provider '{}' does not support embeddings", self.name())
    }
}
