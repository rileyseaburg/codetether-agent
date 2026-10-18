//! Result event delivered from a background `!command` task.

/// Outcome of a background `!command` shell invocation.
/// # Examples
/// ```
/// let event = codetether_chat_shell::ShellEvent {
///     command: "echo ok".into(), output: "ok".into(), success: true, duration_ms: 1,
/// };
/// assert!(event.success);
/// ```
#[derive(Debug)]
pub struct ShellEvent {
    /// The command that was run (for the transcript label).
    pub command: String,
    /// Combined stdout/stderr, already truncated.
    pub output: String,
    /// Whether the command exited successfully.
    pub success: bool,
    /// Wall-clock duration in milliseconds.
    pub duration_ms: u64,
}
