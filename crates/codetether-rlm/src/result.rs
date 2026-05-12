//! RLM processing result type.

use serde::{Deserialize, Serialize};

/// RLM processing result.
///
/// `trace` is populated when the caller supplied a session bus so
/// downstream consumers can reconstruct iteration-by-iteration
/// behaviour. `trace_id` is always generated and echoed on the
/// matching `RlmComplete` session event
/// event for correlation.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::rlm::{RlmResult, RlmStats};
///
/// let r = RlmResult {
///     processed: "summary".into(),
///     stats: RlmStats::default(),
///     success: true,
///     error: None,
///     trace: None,
///     trace_id: None,
/// };
/// assert!(r.success);
/// assert!(r.trace.is_none());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmResult {
    /// The final text produced by the loop (summary or answer).
    pub processed: String,
    /// Aggregate statistics for the run.
    pub stats: super::RlmStats,
    /// `true` when the loop converged within its iteration budget.
    pub success: bool,
    /// Populated when `success` is `false` — a short diagnostic.
    pub error: Option<String>,
    /// Optional per-iteration event trace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<super::context_trace::ContextTrace>,
    /// Identifier echoed on the matching `RlmComplete` bus event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<uuid::Uuid>,
}
