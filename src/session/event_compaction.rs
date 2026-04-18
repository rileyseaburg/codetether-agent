//! Context-compaction payloads for [`SessionEvent`].
//!
//! These types describe the lifecycle of a single compaction attempt and
//! its fallback cascade:
//!
//! ```text
//!   CompactionStarted
//!        │
//!        ▼
//!   ┌──────────┐        success        ┌────────────────────┐
//!   │   RLM    │ ────────────────────▶ │ CompactionCompleted│
//!   └──────────┘                       └────────────────────┘
//!        │ exhausted / failed                   ▲
//!        ▼                                      │ success
//!   ┌──────────────┐   still over budget   ┌────────────┐
//!   │chunk-compress│ ────────────────────▶ │  truncate  │
//!   └──────────────┘                       └────────────┘
//!        │ success                              │ still fails
//!        ▼                                      ▼
//!   CompactionCompleted               CompactionFailed
//! ```
//!
//! [`SessionEvent`]: crate::session::SessionEvent

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Which fallback path the compaction pipeline selected.
///
/// Ordered from highest to lowest fidelity. Emitting this as structured
/// telemetry (rather than a free-form `reason` string) is what lets the
/// trace-driven self-tuning job group runs by strategy.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::FallbackStrategy;
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
    /// [`CompactionFailure`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::FallbackStrategy;
    ///
    /// assert!(FallbackStrategy::Truncate.is_terminal());
    /// assert!(!FallbackStrategy::ChunkCompress.is_terminal());
    /// ```
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Truncate)
    }
}

/// Emitted when a compaction pass begins.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::CompactionStart;
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

/// Emitted when a compaction pass finishes successfully.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::{CompactionOutcome, FallbackStrategy};
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
    /// Correlates with the originating [`CompactionStart`].
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
    /// use codetether_agent::session::{CompactionOutcome, FallbackStrategy};
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

/// Emitted when the terminal truncation fallback fires.
///
/// Distinct from [`CompactionOutcome`] with [`FallbackStrategy::Truncate`]
/// because truncation silently drops context — consumers that care about
/// data loss (the TUI, the flywheel) need an explicit signal to attach
/// user-visible warnings and archive the dropped prefix.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::ContextTruncation;
/// use uuid::Uuid;
///
/// let t = ContextTruncation {
///     trace_id: Uuid::nil(),
///     dropped_tokens: 98_000,
///     kept_messages: 6,
///     archive_ref: Some("minio://codetether/ctx/abc123".into()),
/// };
/// assert!(t.dropped_tokens > 0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTruncation {
    /// Correlates with the originating [`CompactionStart`].
    pub trace_id: Uuid,
    /// How many tokens the truncation removed from the transcript.
    pub dropped_tokens: usize,
    /// Verbatim messages retained after truncation.
    pub kept_messages: usize,
    /// Optional pointer (e.g. MinIO URI) to the archived pre-truncation
    /// prefix so it can be restored with a `/recall` action.
    pub archive_ref: Option<String>,
}

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
/// use codetether_agent::session::{CompactionFailure, FallbackStrategy};
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
    /// Correlates with the originating [`CompactionStart`].
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
