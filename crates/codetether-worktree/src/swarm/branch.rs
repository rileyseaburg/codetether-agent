//! Detect whether a swarm branch contains a net patch before integration.

use crate::{WorktreeInfo, WorktreeManager};
use anyhow::{Context, Result, bail};
use tokio::process::Command;

#[path = "branch/policy.rs"]
pub mod policy;

/// Check whether a worker branch has a patch relative to its merge base.
/// # Arguments
/// * `manager` — Parent repository manager.
/// * `worktree` — Worker branch metadata.
/// # Returns
/// Whether the branch contains a net patch.
/// # Errors
/// Propagates Git merge-base and diff execution errors.
/// # Examples
/// ```no_run
/// # async fn check(manager: &codetether_worktree::WorktreeManager, info: &codetether_worktree::WorktreeInfo) -> anyhow::Result<()> {
/// let _has_patch = codetether_worktree::swarm::branch::has_net_changes(manager, info).await?;
/// # Ok(()) }
/// ```
pub async fn has_net_changes(manager: &WorktreeManager, worktree: &WorktreeInfo) -> Result<bool> {
    let base = Command::new("git")
        .args(["merge-base", "HEAD", &worktree.branch])
        .current_dir(manager.repo_path())
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
        .current_dir(manager.repo_path())
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
#[path = "branch/net_change_fixture.rs"]
mod fixture;
#[cfg(test)]
#[path = "branch/net_change_git.rs"]
mod net_git;
#[cfg(test)]
#[path = "branch_tests.rs"]
mod tests;
