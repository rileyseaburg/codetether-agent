//! Result event delivered from a background `!command` task.

/// Outcome of a background `!command` shell invocation.
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
