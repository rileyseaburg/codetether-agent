//! Aggregated per-provider view. Built by [`super::ProviderMetrics::all_snapshots`].

use serde::{Deserialize, Serialize};

/// Aggregated statistics for one provider across its retained request history.
///
/// All percentiles use the simple `(n * pct) as usize` index method; for the
/// rolling 1000-entry buffer this is accurate to well within 1%.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::ProviderSnapshot;
///
/// let s = ProviderSnapshot {
///     provider: "openai".into(),
///     request_count: 0,
///     total_input_tokens: 0,
///     total_output_tokens: 0,
///     avg_tps: 0.0,
///     avg_latency_ms: 0.0,
///     p50_tps: 0.0,
///     p50_latency_ms: 0.0,
///     p95_tps: 0.0,
///     p95_latency_ms: 0.0,
/// };
/// assert_eq!(s.provider, "openai");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSnapshot {
    /// Provider name.
    pub provider: String,
    /// Requests included in this snapshot.
    pub request_count: usize,
    /// Sum of `input_tokens` across all included requests.
    pub total_input_tokens: u64,
    /// Sum of `output_tokens` across all included requests.
    pub total_output_tokens: u64,
    /// Mean output tokens-per-second.
    pub avg_tps: f64,
    /// Mean end-to-end latency in ms.
    pub avg_latency_ms: f64,
    /// Median output tokens-per-second.
    pub p50_tps: f64,
    /// Median end-to-end latency in ms.
    pub p50_latency_ms: f64,
    /// 95th-percentile tokens-per-second.
    pub p95_tps: f64,
    /// 95th-percentile latency in ms.
    pub p95_latency_ms: f64,
}
