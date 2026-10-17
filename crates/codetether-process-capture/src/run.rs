//! Spawn and capture a command with timeout and process-tree ownership.
use std::io;

use tokio::process::Command;
use tokio::time::{Duration, timeout};

use super::types::CaptureOutcome;

/// Spawn a command and capture a bounded prefix from each output stream.
///
/// # Arguments
/// * `cmd` - Command with piped output and appropriate cancellation setup.
/// * `timeout_secs` - Maximum capture duration in seconds.
/// * `max_bytes` - Maximum retained bytes per stream.
///
/// # Returns
/// Captured completion or a timeout outcome.
///
/// # Errors
/// Returns spawning, waiting, stream reading or reader-task errors.
///
/// # Examples
/// ```rust,no_run
/// # async fn capture() -> std::io::Result<()> {
/// let mut cmd = tokio::process::Command::new("example");
/// cmd.stdout(std::process::Stdio::piped());
/// cmd.stderr(std::process::Stdio::piped());
/// codetether_process_tree::configure(&mut cmd);
/// let _outcome = codetether_process_capture::run(cmd, 30, 8192).await?;
/// # Ok(())
/// # }
/// ```
pub async fn run(
    mut cmd: Command,
    timeout_secs: u64,
    max_bytes: usize,
) -> io::Result<CaptureOutcome> {
    let child = cmd.spawn()?;
    let mut process_tree = codetether_process_tree::Guard::attach(&child);
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
