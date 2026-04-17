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
use serde::{Deserialize, Serialize};

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

    /// Generate embeddings (optional; default returns an error).
    async fn embed(&self, _request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        anyhow::bail!("Provider '{}' does not support embeddings", self.name())
    }
}

/// Metadata about a model offered by a provider.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::ModelInfo;
/// let info = ModelInfo {
///     id: "gpt-4o".into(),
///     name: "GPT-4o".into(),
///     provider: "openai".into(),
///     context_window: 128_000,
///     max_output_tokens: Some(16_384),
///     supports_vision: true,
///     supports_tools: true,
///     supports_streaming: true,
///     input_cost_per_million: Some(2.5),
///     output_cost_per_million: Some(10.0),
/// };
/// assert!(info.supports_vision);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Canonical model ID (e.g. `"us.anthropic.claude-sonnet-4-20250514-v1:0"`).
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Provider that owns this model.
    pub provider: String,
    /// Input + output token budget.
    pub context_window: usize,
    /// Maximum tokens the model can generate in one call.
    pub max_output_tokens: Option<usize>,
    /// Whether the model accepts image inputs.
    pub supports_vision: bool,
    /// Whether the model supports tool/function calling.
    pub supports_tools: bool,
    /// Whether the model supports server-sent streaming.
    pub supports_streaming: bool,
    /// Cost per million input tokens (USD), if known.
    pub input_cost_per_million: Option<f64>,
    /// Cost per million output tokens (USD), if known.
    pub output_cost_per_million: Option<f64>,
}
