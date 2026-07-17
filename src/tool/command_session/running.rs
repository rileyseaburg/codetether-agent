//! Mutable process state retained between tool calls.

use anyhow::{Context, Result};
use tokio::io::AsyncWriteExt;

use super::types::CommandInput;
use super::{Poll, SpawnMetadata};

#[path = "running/attached.rs"]
mod attached;

pub(crate) struct Running {
    pub(super) child: tokio::process::Child,
    pub(super) stdin: Option<CommandInput>,
    pub(super) output: tokio::sync::mpsc::Receiver<Vec<u8>>,
    pub(super) exit_code: Option<i32>,
    pub(super) started: tokio::time::Instant,
    pub metadata: SpawnMetadata,
}

impl Running {
    pub(crate) fn new(mut child: tokio::process::Child, metadata: SpawnMetadata) -> Self {
        let stdin = child
            .stdin
            .take()
            .map(|stdin| Box::pin(stdin) as CommandInput);
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        Self {
            child,
            stdin,
            output: super::readers::start(stdout, stderr),
            exit_code: None,
            started: tokio::time::Instant::now(),
            metadata,
        }
    }

    pub(crate) fn new_attached(
        child: tokio::process::Child,
        metadata: SpawnMetadata,
        terminal: Option<crate::tool::command_pty::Attached>,
    ) -> Result<Self> {
        attached::new(child, metadata, terminal)
    }

    pub(crate) async fn write(&mut self, chars: &str) -> Result<()> {
        let stdin = self.stdin.as_mut().context(
            "stdin is closed for this session; rerun exec_command with tty=true to keep stdin open",
        )?;
        stdin.as_mut().write_all(chars.as_bytes()).await?;
        stdin.as_mut().flush().await?;
        Ok(())
    }

    pub(crate) async fn poll(&mut self, wait_ms: u64, max_bytes: usize) -> Result<Poll> {
        super::drain::poll(self, wait_ms, max_bytes).await
    }
}
