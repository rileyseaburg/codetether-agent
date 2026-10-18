//! Runtime policy guard for Ralph git mutations.

use crate::Host;
use anyhow::{Result, bail};
use serde_json::json;
use std::path::Path;
use std::process::{Command, Output};

pub(crate) fn guard<H: Host>(cwd: &Path, command: &str) -> Result<()> {
    let args = json!({ "command": command, "cwd": cwd.display().to_string() });
    let blocked = match tokio::runtime::Handle::try_current() {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(check::<H>(args))),
        Err(_) => runtime_check::<H>(args),
    };
    if let Some(blocked) = blocked? {
        bail!("git mutation blocked by policy: {}", blocked.output);
    }
    Ok(())
}

/// Perform the existing guarded checkout command.
/// # Arguments
/// Receives the repository directory and requested branch.
/// # Returns
/// The Git process output, including an unsuccessful exit status.
/// # Errors
/// Returns policy rejection, evaluator/runtime or process-launch errors.
/// # Examples
/// ```rust,no_run
/// # fn example<H: codetether_ralph_git::Host>(path: &std::path::Path) -> anyhow::Result<()> {
/// # use codetether_ralph_git::checkout;
/// let output = checkout::<H>(path, "feature")?;
/// assert!(output.status.success());
/// # Ok(()) }
/// ```
pub fn checkout<H: Host>(cwd: &Path, branch: &str) -> Result<Output> {
    guard::<H>(cwd, &format!("git checkout {branch}"))?;
    Ok(Command::new("git")
        .args(["checkout", branch])
        .current_dir(cwd)
        .output()?)
}

async fn check<H: Host>(
    args: serde_json::Value,
) -> Result<Option<codetether_tool_core::ToolResult>> {
    H::check(args).await
}

fn runtime_check<H: Host>(
    args: serde_json::Value,
) -> Result<Option<codetether_tool_core::ToolResult>> {
    Ok(tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(check::<H>(args))?)
}
