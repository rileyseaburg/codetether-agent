//! Required stdio extraction from a spawned language server.

use anyhow::{Result, anyhow};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout};

pub(super) fn take(child: &mut Child) -> Result<(ChildStdout, ChildStderr, ChildStdin)> {
    let stdout = child.stdout.take().ok_or_else(|| anyhow!("No stdout"))?;
    let stderr = child.stderr.take().ok_or_else(|| anyhow!("No stderr"))?;
    let stdin = child.stdin.take().ok_or_else(|| anyhow!("No stdin"))?;
    Ok((stdout, stderr, stdin))
}
