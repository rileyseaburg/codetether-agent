//! Borrowed native-event fields used to update a live trace.
/// Trace-relevant fields borrowed for one synchronous observation.
/// # Examples
/// ```rust
/// use codetether_agent_trace::TraceEvent;
/// let event = TraceEvent::TextChunk("working");
/// assert!(matches!(event, TraceEvent::TextChunk(_)));
/// ```
#[derive(Clone, Copy)]
pub enum TraceEvent<'a> {
    /// Model activity without text yet.
    Thinking,
    /// Latest cumulative assistant draft.
    TextChunk(&'a str),
    /// Completed assistant segment.
    TextComplete(&'a str),
    /// Tool invocation and arguments.
    ToolCallStart { name: &'a str, arguments: &'a str },
    /// Elapsed tool runtime.
    ToolHeartbeat { name: &'a str, elapsed_secs: u64 },
    /// Live output from a running tool.
    ToolOutputChunk { name: &'a str },
    /// Completed tool output and success flag.
    ToolCallComplete {
        name: &'a str,
        output: &'a str,
        success: bool,
    },
    /// Approval is pending.
    ApprovalRequest,
    /// Reported child error.
    Error(&'a str),
    /// Child turn is complete.
    Done,
    /// An event without a live-trace effect.
    Ignored,
}
