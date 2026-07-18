//! Model capability metadata returned by provider catalogs.

use serde::{Deserialize, Serialize};

/// Metadata about a model offered by a provider.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::ModelInfo;
/// let info = ModelInfo {
///     id: "gpt-4o".into(), name: "GPT-4o".into(), provider: "openai".into(),
///     context_window: 128_000, max_output_tokens: Some(16_384),
///     supports_vision: true, supports_tools: true, supports_streaming: true,
///     input_cost_per_million: Some(2.5), output_cost_per_million: Some(10.0),
/// };
/// assert!(info.supports_vision);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Canonical model ID.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Provider that owns the model.
    pub provider: String,
    /// Input and output token budget.
    pub context_window: usize,
    /// Maximum generated tokens.
    pub max_output_tokens: Option<usize>,
    /// Whether image input is accepted.
    pub supports_vision: bool,
    /// Whether tool calls are supported.
    pub supports_tools: bool,
    /// Whether streaming is supported.
    pub supports_streaming: bool,
    /// Input cost per million tokens.
    pub input_cost_per_million: Option<f64>,
    /// Output cost per million tokens.
    pub output_cost_per_million: Option<f64>,
}
