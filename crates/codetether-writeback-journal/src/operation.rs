//! Typed durable writeback operations.
use serde::{Deserialize, Serialize};

/// Typed operation being journalled.
///
/// Extend by adding variants rather than free-form JSON so each new
/// call site declares what it is doing.
/// # Examples
/// ```rust
/// use codetether_writeback_journal::Op;
/// let op = Op::Save;
/// assert!(matches!(op, Op::Save));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Op {
    /// Context-window enforcement rewrote the derived buffer.
    Compaction { before: usize, after: usize },
    /// `Session::save` persisted the transcript to disk.
    Save,
    /// TUI or orchestrator reset the session (e.g., `/clear`).
    Reset,
    /// Lu et al. reset-to-(prompt, summary) context reset (Phase B).
    ContextReset,
    /// MinIO history sink uploaded a delta.
    HistorySinkUpload {
        bucket: String,
        key: String,
        bytes: usize,
    },
}
