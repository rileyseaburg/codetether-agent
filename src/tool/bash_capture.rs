//! Bounded process capture for the bash tool.

#[path = "bash_capture/reader.rs"]
mod reader;
#[path = "bash_capture/types.rs"]
mod types;

use std::io;
use tokio::process::Command;
use tokio::task::JoinHandle;
use tokio::time::{Duration, timeout};
use types::{CapturedOutput, CapturedStream};

pub(super) use types::CaptureOutcome;

type ReaderTask = JoinHandle<io::Result<CapturedStream>>;

pub(super) async fn run(
    mut cmd: Command,
    timeout_secs: u64,
    max_bytes: usize,
) -> io::Result<CaptureOutcome> {
    let child = cmd.spawn()?;
    let capture = capture_child(child, max_bytes);
    match timeout(Duration::from_secs(timeout_secs), capture).await {
        Ok(result) => result.map(CaptureOutcome::Finished),
        Err(_) => Ok(CaptureOutcome::TimedOut),
    }
}

async fn capture_child(
    mut child: tokio::process::Child,
    max_bytes: usize,
) -> io::Result<CapturedOutput> {
    let stdout = child
        .stdout
        .take()
        .map(|s| tokio::spawn(reader::read_limited(s, max_bytes)));
    let stderr = child
        .stderr
        .take()
        .map(|s| tokio::spawn(reader::read_limited(s, max_bytes)));
    let status = child.wait().await?;
    Ok(CapturedOutput {
        status,
        stdout: join_reader(stdout).await?,
        stderr: join_reader(stderr).await?,
    })
}

async fn join_reader(handle: Option<ReaderTask>) -> io::Result<CapturedStream> {
    match handle {
        Some(handle) => handle
            .await
            .map_err(|e| io::Error::other(format!("output reader task failed: {e}")))?,
        None => Ok(CapturedStream::empty()),
    }
}
