//! Non-building Git health check for speculative branch probes.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub(super) fn static_clean(worktree: &Path) -> Result<bool> {
    let output = Command::new("git")
        .args(["diff", "--check"])
        .current_dir(worktree)
        .output()
        .context("Failed to run static Git branch check")?;
    Ok(output.status.success())
}
