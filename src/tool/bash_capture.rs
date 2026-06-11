//! Bounded process capture for the bash tool.

#[path = "bash_capture/reader.rs"]
mod reader;
#[path = "bash_capture/run.rs"]
mod run_impl;
#[path = "bash_capture/types.rs"]
mod types;

use std::io;
use tokio::task::JoinHandle;
use types::{CapturedOutput, CapturedStream};

pub(super) use run_impl::run;
pub(super) use types::CaptureOutcome;

type ReaderTask = JoinHandle<io::Result<CapturedStream>>;

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
