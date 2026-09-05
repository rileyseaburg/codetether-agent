//! Resolve explicit start points to immutable commit IDs before allocation.

use super::WorktreeManager;
use anyhow::{Context, Result, bail};

pub(super) async fn resolve(manager: &WorktreeManager, revision: &str) -> Result<String> {
    let output = tokio::process::Command::new("git")
        .args(["rev-parse", "--verify", "--end-of-options"])
        .arg(format!("{revision}^{{commit}}"))
        .current_dir(&manager.repo_path)
        .output()
        .await
        .context("Failed to resolve worktree start point")?;
    if !output.status.success() {
        bail!(
            "Invalid worktree start point '{revision}': {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8(output.stdout)
        .context("Git returned a non-UTF-8 commit ID")?
        .trim()
        .to_string())
}
