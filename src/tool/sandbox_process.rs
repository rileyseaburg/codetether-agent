use anyhow::{Context, Result, anyhow};
use std::process::Output;
use tokio::process::Command;

pub(super) async fn wait(
    mut cmd: Command,
    timeout_secs: u64,
    violations: &mut Vec<String>,
) -> Result<Output> {
    let timeout = std::time::Duration::from_secs(timeout_secs);
    crate::tool::process_tree::configure(&mut cmd);
    let child = cmd.spawn().context("Failed to spawn sandboxed process")?;
    let mut process_tree = crate::tool::process_tree::Guard::attach(&child);
    match tokio::time::timeout(timeout, child.wait_with_output()).await {
        Ok(Ok(output)) => {
            process_tree.disarm();
            Ok(output)
        }
        Ok(Err(error)) => Err(error).context("Failed to wait for sandboxed process"),
        Err(_) => {
            violations.push("timeout_exceeded".to_string());
            Err(anyhow!("Sandboxed process timed out after {timeout_secs}s"))
        }
    }
}
