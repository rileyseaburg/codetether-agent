//! Snapshot data exposed to live-agent observers.
use crate::LiveTraceEntry;

/// Read-only in-flight trace exposed to TUI renderers.
///
/// # Examples
///
/// ```rust
/// use codetether_agent_trace::LiveTraceSnapshot;
/// let trace = LiveTraceSnapshot::default();
/// assert!(trace.entries.is_empty());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LiveTraceSnapshot {
    /// User request currently being processed.
    pub prompt: String,
    /// Completed events emitted during the current turn.
    pub entries: Vec<LiveTraceEntry>,
    /// Latest cumulative assistant text received before completion.
    pub streaming_text: Option<String>,
    /// Human-readable description of the current operation.
    pub activity: Option<String>,
}
