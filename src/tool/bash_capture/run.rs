use std::io;

use tokio::process::Command;
use tokio::time::{Duration, timeout};

use super::types::CaptureOutcome;

pub(in crate::tool::bash) async fn run(
    mut cmd: Command,
    timeout_secs: u64,
    max_bytes: usize,
) -> io::Result<CaptureOutcome> {
    let child = cmd.spawn()?;
    let mut process_tree = crate::tool::process_tree::Guard::attach(&child);
    let capture = super::capture_child(child, max_bytes);
    match timeout(Duration::from_secs(timeout_secs), capture).await {
        Ok(Ok(result)) => {
            process_tree.disarm();
            Ok(CaptureOutcome::Finished(result))
        }
        Ok(Err(error)) => Err(error),
        Err(_) => Ok(CaptureOutcome::TimedOut),
    }
}
