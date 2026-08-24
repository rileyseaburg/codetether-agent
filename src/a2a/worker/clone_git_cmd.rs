//! Low-level git command execution.

use std::path::Path;

use anyhow::{Context, Result};

/// Run a git command at an optional working directory.
pub(super) async fn run_git_command_at(
    current_dir: Option<&Path>,
    args: Vec<String>,
) -> Result<String> {
    crate::tool::network_access::require("a2a_git", &serde_json::Value::Null)?;
    let cwd = current_dir
        .map(Path::to_path_buf)
        .unwrap_or(std::env::current_dir().context("Failed to resolve Git workspace")?);
    let output = crate::tool::git::process::output_networked(&cwd, &args, &[], true)
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

#[cfg(test)]
#[path = "clone_git_cmd_tests.rs"]
mod tests;
