//! Value types crossing the persistent-command lifecycle boundaries.

use std::path::PathBuf;
use std::pin::Pin;
use std::time::Duration;

pub(super) type CommandInput = Pin<Box<dyn tokio::io::AsyncWrite + Send>>;

/// Host-supplied execution context used when rendering command results.
///
/// # Examples
/// ```rust
/// let metadata = codetether_command_session::SpawnMetadata {
///     sandboxed: true, interactive: false, redactions: vec![],
///     unsafe_fallbacks: vec![], cwd: ".".into(),
/// };
/// assert!(metadata.sandboxed);
/// ```
pub struct SpawnMetadata {
    /// Whether the host spawned the command in its sandbox.
    pub sandboxed: bool,
    /// Whether the command accepts interactive terminal input.
    pub interactive: bool,
    /// Secret strings to remove from model-facing output.
    pub redactions: Vec<String>,
    /// Host-reported fallback descriptions attached to results.
    pub unsafe_fallbacks: Vec<String>,
    /// Working directory selected by the host.
    pub cwd: PathBuf,
}

/// Incremental output, exit status and liveness observed during one poll.
///
/// Values are returned by [crate::Running::poll].
///
/// # Examples
/// ```rust
/// use std::time::Duration;
/// let poll = codetether_command_session::Poll {
///     output: String::new(), running: true, exit_code: None,
///     elapsed: Duration::ZERO, omitted_bytes: 0, poll_bytes: 0,
///     silent_for: Duration::ZERO, session_bytes: 0,
/// };
/// assert!(poll.running);
/// ```
pub struct Poll {
    /// Retained head and tail of output collected by this poll.
    pub output: String,
    /// Whether the child is still running.
    pub running: bool,
    /// Child exit code, or `None` while still running.
    pub exit_code: Option<i32>,
    /// Time since this command session started.
    pub elapsed: Duration,
    /// Bytes omitted from this poll by its retention budget.
    pub omitted_bytes: usize,
    /// Bytes observed during this poll only; zero means the poll was silent.
    pub poll_bytes: usize,
    /// Time since the session last produced any output.
    pub silent_for: Duration,
    /// Bytes produced by the session across all polls.
    pub session_bytes: usize,
}
