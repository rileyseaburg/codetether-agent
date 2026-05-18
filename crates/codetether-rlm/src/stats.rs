//! RLM processing statistics.

use serde::{Deserialize, Serialize};

/// Statistics collected during a single RLM processing run.
///
/// Returned inside [`RlmResult`](super::RlmResult) and emitted on the
/// session bus as part of the
/// `RlmComplete` event.
///
/// # Examples
///
/// ```rust
/// use codetether_rlm::RlmStats;
///
/// let s = RlmStats::default();
/// assert_eq!(s.input_tokens, 0);
/// assert_eq!(s.iterations, 0);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RlmStats {
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub iterations: usize,
    pub subcalls: usize,
    pub elapsed_ms: u64,
    pub compression_ratio: f64,
}
