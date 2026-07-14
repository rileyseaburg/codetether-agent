//! Detect whether a swarm branch contains a net patch before integration.

use crate::worktree::{WorktreeInfo, WorktreeManager};
use anyhow::{Context, Result, bail};
use tokio::process::Command;

#[path = "worktree_branch/policy.rs"]
pub(crate) mod policy;

pub(crate) async fn has_net_changes(
    manager: &WorktreeManager,
    worktree: &WorktreeInfo,
) -> Result<bool> {
    let base = Command::new("git")
        .args(["merge-base", "HEAD", &worktree.branch])
        .current_dir(&manager.repo_path)
        .output()
        .await
        .context("failed to find swarm branch merge base")?;
    if !base.status.success() {
        bail!(
            "git merge-base failed: {}",
            String::from_utf8_lossy(&base.stderr)
        );
    }
    let base = String::from_utf8_lossy(&base.stdout).trim().to_string();
    let status = Command::new("git")
        .args(["diff", "--quiet", &base, &worktree.branch, "--"])
        .current_dir(&manager.repo_path)
        .status()
        .await
        .context("failed to inspect swarm branch patch")?;
    match status.code() {
        Some(0) => Ok(false),
        Some(1) => Ok(true),
        _ => bail!("git diff failed with status {status}"),
    }
}

#[cfg(test)]
#[path = "worktree_branch/net_change_fixture.rs"]
mod fixture;
#[cfg(test)]
#[path = "worktree_branch/net_change_git.rs"]
mod net_git;
#[cfg(test)]
#[path = "worktree_branch_tests.rs"]
mod tests;
