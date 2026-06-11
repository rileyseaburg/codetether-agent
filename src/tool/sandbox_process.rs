use anyhow::{Context, Result, anyhow};
use std::process::Output;
use tokio::process::Command;

pub(super) async fn wait(
    mut cmd: Command,
    timeout_secs: u64,
    violations: &mut Vec<String>,
) -> Result<Output> {
    let timeout = std::time::Duration::from_secs(timeout_secs);
    cmd.kill_on_drop(true);
    let child = cmd.spawn().context("Failed to spawn sandboxed process")?;
    tokio::time::timeout(timeout, child.wait_with_output())
        .await
        .map_err(|_| {
            violations.push("timeout_exceeded".to_string());
            anyhow!("Sandboxed process timed out after {timeout_secs}s")
        })?
        .context("Failed to wait for sandboxed process")
}
