//! Owned, isolated RustyRoad process and pipe lifecycle.

use super::{command::command, rpc::Rpc};
use crate::tool::process_tree;
use anyhow::{Context, Result};
use std::{path::Path, process::Stdio};
use tokio::{
    io::BufReader,
    process::{Child, Command},
};

pub(super) struct Process {
    _tree: process_tree::Guard,
    pub(super) child: Child,
    pub(super) rpc: Rpc,
}

impl Process {
    pub(super) async fn spawn(cwd: &Path, environment: &str) -> Result<Self> {
        Self::spawn_command(command(cwd, environment)).await
    }
    pub(super) async fn spawn_command(mut command: Command) -> Result<Self> {
        process_tree::configure(&mut command);
        let mut child = command.stdin(Stdio::piped()).stdout(Stdio::piped())
            .stderr(Stdio::null()).kill_on_drop(true).spawn()
            .context("Unable to start rustyroad-mcp. Install with: cargo install rustyroad --locked --bin rustyroad-mcp")?;
        let tree = process_tree::Guard::attach(&child);
        let stdin = child.stdin.take().context("RustyRoad stdin unavailable")?;
        let stdout = child
            .stdout
            .take()
            .context("RustyRoad stdout unavailable")?;
        Ok(Self {
            _tree: tree,
            child,
            rpc: Rpc {
                stdin,
                stdout: BufReader::new(stdout),
                next_id: 0,
            },
        })
    }
    pub(super) async fn close(&mut self) -> Result<()> {
        // kill() also waits/reaps. Drop kills on cancellation or initialization error.
        if self.child.try_wait()?.is_none() {
            self.child.kill().await?;
        }
        Ok(())
    }
}
