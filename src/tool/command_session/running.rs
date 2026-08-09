//! Mutable process state retained between tool calls.

use anyhow::Result;

use super::activity::Activity;
use super::types::CommandInput;
use super::{Poll, SpawnMetadata};

#[path = "running/attached.rs"]
mod attached;
#[path = "running/write.rs"]
mod write;

pub(crate) struct Running {
    pub(super) child: tokio::process::Child,
    pub(super) stdin: Option<CommandInput>,
    pub(super) output: tokio::sync::mpsc::Receiver<Vec<u8>>,
    pub(super) exit_code: Option<i32>,
    pub(super) activity: Activity,
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
            activity: Activity::new(),
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
        write::write(self, chars).await
    }

    pub(crate) async fn poll(&mut self, wait_ms: u64, max_bytes: usize) -> Result<Poll> {
        super::drain::poll(self, wait_ms, max_bytes).await
    }
}
