//! Phase timings; never report cold-request throughput as warm decode TPS.
use serde::{Deserialize, Serialize};
/// Measured wall-clock generation phases, with CUDA synchronized at boundaries.
/// # Examples
/// ```rust
/// use codetether_agent::provider::bonsai::GenerationTiming;
/// let timing = GenerationTiming { generated_tokens: 11, decode_ms: 1000.0, ..Default::default() };
/// assert_eq!(timing.decode_tps(), Some(10.0));
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GenerationTiming {
    /// Checkpoint/device initialization (zero when weights are already resident).
    pub load_ms: f64,
    /// Prompt execution, excluding the first sampling operation.
    pub prefill_ms: f64,
    /// Request-to-first-sampled-token, excluding model loading.
    pub ttft_ms: f64,
    /// Time between the first and last sampled non-EOS token.
    pub decode_ms: f64,
    /// Number of generated non-EOS token IDs.
    pub generated_tokens: usize,
    /// Whole generation wall time, excluding model loading.
    pub total_ms: f64,
}
impl GenerationTiming {
    /// Output-token intervals per second; undefined for fewer than two tokens.
    pub fn decode_tps(&self) -> Option<f64> {
        (self.generated_tokens > 1 && self.decode_ms > 0.0)
            .then(|| (self.generated_tokens - 1) as f64 * 1000.0 / self.decode_ms)
    }
}
