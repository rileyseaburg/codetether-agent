//! Shared FallbackStrategy event payload and operations.
use serde::{Deserialize, Serialize};

/// Which fallback path the compaction pipeline selected.
///
/// Ordered from highest to lowest fidelity. Emitting this as structured
/// telemetry (rather than a free-form `reason` string) is what lets the
/// trace-driven self-tuning job group runs by strategy.
///
/// # Examples
///
/// ```rust
/// use codetether_session_events::FallbackStrategy;
///
/// assert_eq!(FallbackStrategy::Rlm.as_str(), "rlm");
/// assert!(FallbackStrategy::Truncate.is_terminal());
/// assert!(!FallbackStrategy::Rlm.is_terminal());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackStrategy {
    /// Recursive Language Model summarisation (preferred path).
    Rlm,
    /// Deterministic chunk-based compression (when RLM is unavailable or
    /// exhausted without converging).
    ChunkCompress,
    /// Hard truncation to a fixed fraction of the budget. Terminal
    /// successful fallback — guarantees a valid request at the cost of
    /// silently dropping older context.
    Truncate,
}

impl FallbackStrategy {
    /// Stable wire-format identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rlm => "rlm",
            Self::ChunkCompress => "chunk_compress",
            Self::Truncate => "truncate",
        }
    }

    /// Returns `true` when this strategy is the last resort before
    /// [`crate::CompactionFailure`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_session_events::FallbackStrategy;
    ///
    /// assert!(FallbackStrategy::Truncate.is_terminal());
    /// assert!(!FallbackStrategy::ChunkCompress.is_terminal());
    /// ```
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Truncate)
    }
}
