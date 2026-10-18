//! Commit dirty swarm worktree edits before branch integration.

#[path = "inventory.rs"]
pub mod inventory;
#[path = "inventory_parse.rs"]
mod inventory_parse;

use super::git::run as git;
use crate::WorktreeInfo;
use anyhow::{Result, bail};
use codetether_provenance::{ExecutionOrigin, ExecutionProvenance, git_commit_with_provenance};

/// Commit pending swarm edits with execution provenance.
/// # Arguments
/// * `info` — Allocated worktree.
/// * `task_id` — Worker identity recorded in the commit.
/// # Returns
/// Success after committing, or immediately if the worktree is clean.
/// # Errors
/// Propagates status, staging, or provenance-aware commit errors.
/// # Examples
/// ```no_run
/// # async fn prepare(info: &codetether_worktree::WorktreeInfo) -> anyhow::Result<()> {
/// codetether_worktree::swarm::commit::prepare(info, "task-1").await?;
/// # Ok(()) }
/// ```
pub async fn prepare(info: &WorktreeInfo, task_id: &str) -> Result<()> {
    let status = git(&info.path, &["status", "--porcelain"]).await?;
    if !status.status.success() {
        bail!(
            "git status failed: {}",
            String::from_utf8_lossy(&status.stderr)
        );
    }
    if String::from_utf8_lossy(&status.stdout).trim().is_empty() {
        return Ok(());
    }
    let add = git(&info.path, &["add", "--all"]).await?;
    if !add.status.success() {
        bail!("git add failed: {}", String::from_utf8_lossy(&add.stderr));
    }
    let provenance = ExecutionProvenance::for_operation(task_id, ExecutionOrigin::Swarm);
    let message = format!("feat: complete swarm subtask {task_id}");
    let commit = git_commit_with_provenance(&info.path, &message, Some(&provenance)).await?;
    if !commit.status.success() {
        bail!(
            "git commit failed: {}",
            String::from_utf8_lossy(&commit.stderr)
        );
    }
    tracing::info!(subtask_id = %task_id, branch = %info.branch, "Committed swarm worktree changes");
    Ok(())
}

#[cfg(test)]
#[path = "commit_tests.rs"]
mod tests;
