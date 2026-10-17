//! Event effects consumed by the child-output accumulator.
/// Relevant native event effects; other progress still resets inactivity.
/// # Examples
/// ```rust
/// use codetether_child_events::Event;
/// let event: Event<()> = Event::TextComplete("done".into());
/// assert!(matches!(event, Event::TextComplete(_)));
/// ```
pub enum Event<A> {
    /// Completed assistant response segment.
    TextComplete(String),
    /// Completed reasoning segment.
    ThinkingComplete(String),
    /// Completed tool output used for the result preview.
    ToolCallComplete {
        name: String,
        output: String,
        success: bool,
    },
    /// Terminal or recoverable execution error text.
    Error(String),
    /// Native approval request to handle immediately.
    ApprovalRequest(A),
    /// End of the native event stream.
    Done,
    /// Progress without an output accumulator effect.
    Ignored,
}
