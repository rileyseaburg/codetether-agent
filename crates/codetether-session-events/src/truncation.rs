//! Shared ContextTruncation event payload and operations.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Emitted when the terminal truncation fallback fires.
///
/// Distinct from [`crate::CompactionOutcome`] with [`crate::FallbackStrategy::Truncate`]
/// because truncation silently drops context — consumers that care about
/// data loss (the TUI, the flywheel) need an explicit signal to attach
/// user-visible warnings and archive the dropped prefix.
///
/// # Examples
///
/// ```rust
/// use codetether_session_events::ContextTruncation;
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
    /// Correlates with the originating [`crate::CompactionStart`].
    pub trace_id: Uuid,
    /// How many tokens the truncation removed from the transcript.
    pub dropped_tokens: usize,
    /// Verbatim messages retained after truncation.
    pub kept_messages: usize,
    /// Optional pointer (e.g. MinIO URI) to the archived pre-truncation
    /// prefix so it can be restored with a `/recall` action.
    pub archive_ref: Option<String>,
}
