/// Borrowed session and outcome fields for one archive event.
/// # Examples
/// ```
/// let record = codetether_event_stream::tool_archive::Record {
///     session_id: "session", message_count: 1, tool_name: "read",
///     success: true, duration_ms: 2, rendered: "contents",
/// };
/// assert!(record.success);
/// ```
pub struct Record<'a> {
    /// Parent session's durable identity.
    pub session_id: &'a str,
    /// Transcript length used for the archive sequence range.
    pub message_count: u64,
    /// Canonical tool name.
    pub tool_name: &'a str,
    /// Execution result status.
    pub success: bool,
    /// Execution duration in milliseconds.
    pub duration_ms: u64,
    /// Rendered tool result serialized into the event.
    pub rendered: &'a str,
}
