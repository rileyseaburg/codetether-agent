//! Shared CompactionOutcome event payload and operations.
use crate::FallbackStrategy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Emitted when a compaction pass finishes successfully.
///
/// # Examples
///
/// ```rust
/// use codetether_session_events::{CompactionOutcome, FallbackStrategy};
/// use uuid::Uuid;
///
/// let o = CompactionOutcome {
///     trace_id: Uuid::nil(),
///     strategy: FallbackStrategy::Rlm,
///     before_tokens: 140_000,
///     after_tokens: 22_400,
///     kept_messages: 12,
/// };
/// assert!(o.reduction() > 0.8);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionOutcome {
    /// Correlates with the originating [`crate::CompactionStart`].
    pub trace_id: Uuid,
    /// Which strategy ultimately produced the final context.
    pub strategy: FallbackStrategy,
    /// Estimated request tokens before compaction.
    pub before_tokens: usize,
    /// Estimated request tokens after compaction.
    pub after_tokens: usize,
    /// Verbatim messages retained in the compacted transcript.
    pub kept_messages: usize,
}

impl CompactionOutcome {
    /// `1.0 - (after / before)`, clamped to `[0.0, 1.0]`.
    ///
    /// A reduction of `0.9` means 90 % of tokens were removed. Returns
    /// `0.0` for pathological inputs (`before_tokens == 0`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_session_events::{CompactionOutcome, FallbackStrategy};
    /// use uuid::Uuid;
    ///
    /// let o = CompactionOutcome {
    ///     trace_id: Uuid::nil(),
    ///     strategy: FallbackStrategy::Rlm,
    ///     before_tokens: 1000, after_tokens: 100, kept_messages: 0,
    /// };
    /// assert!((o.reduction() - 0.9).abs() < 1e-9);
    /// ```
    pub fn reduction(&self) -> f64 {
        if self.before_tokens == 0 {
            0.0
        } else {
            (1.0 - self.after_tokens as f64 / self.before_tokens as f64).clamp(0.0, 1.0)
        }
    }
}
