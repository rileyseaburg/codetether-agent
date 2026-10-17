//! Mutable process state retained between tool calls.

use anyhow::Result;

use super::activity::Activity;
use super::types::CommandInput;
use super::{Poll, SpawnMetadata};

#[path = "running/attached.rs"]
mod attached;
#[path = "running/write.rs"]
mod write;

/// Child-process state retained between writes and incremental output polls.
///
/// # Examples
/// ```rust,no_run
/// # fn retain(child: tokio::process::Child, metadata: codetether_command_session::SpawnMetadata) {
/// let _running = codetether_command_session::Running::new(child, metadata);
/// # }
/// ```
pub struct Running {
    pub(super) process_tree: codetether_process_tree::Guard,
    pub(super) child: tokio::process::Child,
    pub(super) stdin: Option<CommandInput>,
    pub(super) output: tokio::sync::mpsc::Receiver<Vec<u8>>,
    pub(super) exit_code: Option<i32>,
    pub(super) activity: Activity,
    /// Execution context and response redactions supplied by the spawning host.
    pub metadata: SpawnMetadata,
}

impl Running {
    /// Retain a child with piped streams and begin forwarding its output.
    pub fn new(mut child: tokio::process::Child, metadata: SpawnMetadata) -> Self {
        let stdin = child
            .stdin
            .take()
            .map(|stdin| Box::pin(stdin) as CommandInput);
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        Self {
            process_tree: codetether_process_tree::Guard::attach(&child),
            child,
            stdin,
            output: super::readers::start(stdout, stderr),
            exit_code: None,
            activity: Activity::new(),
            metadata,
        }
    }

    /// Retain a child and attach its PTY, or use its existing pipe streams.
    /// # Errors
    /// Returns an error if cloning the PTY descriptor fails.
    pub fn new_attached(
        child: tokio::process::Child,
        metadata: SpawnMetadata,
        terminal: Option<codetether_command_pty::Attached>,
    ) -> Result<Self> {
        attached::new(child, metadata, terminal)
    }

    /// Write and flush input to the child.
    /// # Errors
    /// Returns an error when no input stream remains or writing fails.
    pub async fn write(&mut self, chars: &str) -> Result<()> {
        write::write(self, chars).await
    }

    /// Collect output up to `wait_ms`, retaining at most `max_bytes` bytes.
    /// # Errors
    /// Returns an error if querying the child exit status fails.
    pub async fn poll(&mut self, wait_ms: u64, max_bytes: usize) -> Result<Poll> {
        super::drain::poll(self, wait_ms, max_bytes).await
    }
}
