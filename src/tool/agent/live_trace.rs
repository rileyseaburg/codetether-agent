//! In-flight transcript state for TUI observation of managed agents.

#[path = "live_trace_record.rs"]
mod record;
#[path = "live_trace_store.rs"]
mod store;
#[path = "live_trace_tool.rs"]
mod tool;
pub(in crate::tool::agent) use store::{begin, clear, observe, snapshot};

/// A completed item in an agent's live trace.
///
/// Variants represent assistant text, tool starts, tool results, and errors.
///
/// # Examples
///
/// ```ignore
/// let entry = LiveTraceEntry::Assistant("done".into());
/// assert!(matches!(entry, LiveTraceEntry::Assistant(_)));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum LiveTraceEntry {
    /// A completed assistant response segment.
    Assistant(String),
    /// A tool invocation and its serialized arguments.
    ToolCall { name: String, arguments: String },
    /// A completed tool invocation with bounded output.
    ToolResult {
        name: String,
        output: String,
        success: bool,
    },
    /// A terminal error reported by the child session.
    Error(String),
}

/// Read-only in-flight trace exposed to TUI renderers.
///
/// # Examples
///
/// ```ignore
/// let trace = LiveTraceSnapshot::default();
/// assert!(trace.entries.is_empty());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LiveTraceSnapshot {
    /// User request currently being processed.
    pub prompt: String,
    /// Completed events emitted during the current turn.
    pub entries: Vec<LiveTraceEntry>,
    /// Latest cumulative assistant text received before completion.
    pub streaming_text: Option<String>,
    /// Human-readable description of the current operation.
    pub activity: Option<String>,
}
