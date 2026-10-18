//! Validate child identity and take its captured output streams.

use anyhow::{Context, Result};
use tokio::process::{Child, ChildStderr, ChildStdout};

pub(super) fn take(child: &mut Child) -> Result<(u32, ChildStdout, ChildStderr)> {
    let pid = child.id().context("mux agent process has no pid")?;
    let stdout = child
        .stdout
        .take()
        .context("mux agent stdout unavailable")?;
    let stderr = child
        .stderr
        .take()
        .context("mux agent stderr unavailable")?;
    Ok((pid, stdout, stderr))
}
