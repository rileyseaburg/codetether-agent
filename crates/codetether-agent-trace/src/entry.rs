//! Completed entries retained in the live trace.

/// A completed item in an agent's live trace.
///
/// Variants represent assistant text, tool starts, tool results, and errors.
///
/// # Examples
///
/// ```rust
/// use codetether_agent_trace::LiveTraceEntry;
/// let entry = LiveTraceEntry::Assistant("done".into());
/// assert!(matches!(entry, LiveTraceEntry::Assistant(_)));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiveTraceEntry {
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
