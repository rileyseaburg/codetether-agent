//! Bounded output capture for cancellable child processes.
//!
//! [run] spawns a configured command, drains both output streams and enforces
//! a timeout. [CaptureOutcome] reports completion or timeout; captured records
//! retain byte totals even when their stored text is truncated.

#[path = "reader.rs"]
mod reader;
#[path = "run.rs"]
mod run_impl;
#[path = "types.rs"]
mod types;

use std::io;
use tokio::task::JoinHandle;
pub use types::{CapturedOutput, CapturedStream};

pub use run_impl::run;
pub use types::CaptureOutcome;

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
