//! Shared CompactionStart event payload and operations.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Emitted when a compaction pass begins.
///
/// # Examples
///
/// ```rust
/// use codetether_session_events::CompactionStart;
/// use uuid::Uuid;
///
/// let s = CompactionStart {
///     trace_id: Uuid::nil(),
///     reason: "context_budget".into(),
///     before_tokens: 140_000,
///     budget: 128_000,
/// };
/// assert!(s.before_tokens > s.budget);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionStart {
    /// Correlates subsequent completion / failure / truncation events.
    pub trace_id: Uuid,
    /// Short machine-readable reason (e.g. `"context_budget"`, `"user_requested"`).
    pub reason: String,
    /// Estimated request tokens *before* compaction runs.
    pub before_tokens: usize,
    /// Usable budget the compaction is trying to fit under.
    pub budget: usize,
}
