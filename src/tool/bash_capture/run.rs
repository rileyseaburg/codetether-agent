use std::io;

use tokio::process::Command;
use tokio::time::{Duration, timeout};

use super::types::CaptureOutcome;

pub(in crate::tool::bash) async fn run(
    mut cmd: Command,
    timeout_secs: u64,
    max_bytes: usize,
) -> io::Result<CaptureOutcome> {
    cmd.kill_on_drop(true);
    let child = cmd.spawn()?;
    let capture = super::capture_child(child, max_bytes);
    match timeout(Duration::from_secs(timeout_secs), capture).await {
        Ok(result) => result.map(CaptureOutcome::Finished),
        Err(_) => Ok(CaptureOutcome::TimedOut),
    }
}
