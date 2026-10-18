//! Shared CompactionFailure event payload and operations.
use crate::FallbackStrategy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Emitted when *every* fallback strategy failed to fit under budget.
///
/// At this point the caller must surface an error to the user — sending
/// the request to the provider would 400. `fell_back_to` is `None`
/// explicitly (rather than omitted) so JSONL consumers can distinguish
/// "never attempted truncation" from "attempted and it also failed".
///
/// # Examples
///
/// ```rust
/// use codetether_session_events::{CompactionFailure, FallbackStrategy};
/// use uuid::Uuid;
///
/// let f = CompactionFailure {
///     trace_id: Uuid::nil(),
///     fell_back_to: Some(FallbackStrategy::Truncate),
///     reason: "truncation below minimum viable request".into(),
///     after_tokens: 9_200,
///     budget: 8_000,
/// };
/// assert!(f.after_tokens > f.budget);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionFailure {
    /// Correlates with the originating [`crate::CompactionStart`].
    pub trace_id: Uuid,
    /// Last strategy attempted before giving up, or `None` if compaction
    /// never managed to run at all.
    pub fell_back_to: Option<FallbackStrategy>,
    /// Human-readable diagnostic.
    pub reason: String,
    /// Final estimated request tokens after the last attempt.
    pub after_tokens: usize,
    /// Budget the request still fails to fit under.
    pub budget: usize,
}
