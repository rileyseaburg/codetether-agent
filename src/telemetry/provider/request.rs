//! Single provider request record with timing and token counts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// One LLM provider request. Push into [`super::ProviderMetrics`] via
/// [`super::ProviderMetrics::record`].
///
/// Note: `prompt_tokens` / `completion_tokens` and `input_tokens` /
/// `output_tokens` carry the same numbers — the duplication exists because
/// different downstream consumers were written against different field names.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::ProviderRequestRecord;
/// use chrono::Utc;
///
/// let r = ProviderRequestRecord {
///     provider: "anthropic".into(),
///     model: "claude-sonnet-4".into(),
///     timestamp: Utc::now(),
///     prompt_tokens: 1_000,
///     completion_tokens: 500,
///     input_tokens: 1_000,
///     output_tokens: 500,
///     latency_ms: 2_000,
///     ttft_ms: Some(300),
///     success: true,
/// };
/// assert!((r.tokens_per_second() - 250.0).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRequestRecord {
    /// Provider name (e.g. `"anthropic"`, `"openai"`).
    pub provider: String,
    /// Model id (e.g. `"claude-sonnet-4"`).
    pub model: String,
    /// When the request started (or completed — callers are consistent).
    pub timestamp: DateTime<Utc>,
    /// Input tokens (provider's `prompt_tokens` field).
    pub prompt_tokens: u64,
    /// Output tokens (provider's `completion_tokens` field).
    pub completion_tokens: u64,
    /// Duplicate of `prompt_tokens` under the wire-format name.
    pub input_tokens: u64,
    /// Duplicate of `completion_tokens` under the wire-format name.
    pub output_tokens: u64,
    /// End-to-end latency in milliseconds.
    pub latency_ms: u64,
    /// Time-to-first-token in milliseconds, when the provider streamed.
    pub ttft_ms: Option<u64>,
    /// `true` iff the provider returned a non-error response.
    pub success: bool,
}

impl ProviderRequestRecord {
    /// Output tokens per second over the full request latency. Returns `0.0`
    /// when `latency_ms` is zero, never panics.
    pub fn tokens_per_second(&self) -> f64 {
        if self.latency_ms == 0 {
            return 0.0;
        }
        (self.output_tokens as f64) / (self.latency_ms as f64 / 1000.0)
    }
}
