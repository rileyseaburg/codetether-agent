//! Value types crossing the persistent-command lifecycle boundaries.

use std::path::PathBuf;
use std::pin::Pin;
use std::time::Duration;

pub(super) type CommandInput = Pin<Box<dyn tokio::io::AsyncWrite + Send>>;

pub(crate) struct SpawnMetadata {
    pub sandboxed: bool,
    pub interactive: bool,
    pub redactions: Vec<String>,
    pub unsafe_fallbacks: Vec<String>,
    pub cwd: PathBuf,
}

pub(crate) struct Poll {
    pub output: String,
    pub running: bool,
    pub exit_code: Option<i32>,
    pub elapsed: Duration,
    pub omitted_bytes: usize,
    /// Bytes observed during this poll only; zero means the poll was silent.
    pub poll_bytes: usize,
    /// Time since the session last produced any output.
    pub silent_for: Duration,
    /// Bytes produced by the session across all polls.
    pub session_bytes: usize,
}
