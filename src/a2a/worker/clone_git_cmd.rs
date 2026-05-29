//! Low-level git command execution.

use std::path::Path;

use anyhow::{Context, Result};
use tokio::process::Command;

/// Run a git command at an optional working directory.
pub(super) async fn run_git_command_at(
    current_dir: Option<&Path>,
    args: Vec<String>,
) -> Result<String> {
    let mut command = Command::new("git");
    if let Some(dir) = current_dir {
        command.current_dir(dir);
    }
    let output = command
        .args(args.iter().map(String::as_str))
        .output()
        .await
        .context("Failed to execute git command")?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
    }
    Err(anyhow::anyhow!(
        "Git command failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}
