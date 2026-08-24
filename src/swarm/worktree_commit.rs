//! Commit dirty swarm worktree edits before branch integration.

#[path = "worktree_inventory.rs"]
pub(crate) mod inventory;
#[path = "worktree_inventory_parse.rs"]
mod inventory_parse;

#[path = "worktree_commit_git.rs"]
mod git_process;

use crate::provenance::{ExecutionOrigin, ExecutionProvenance, git_commit_with_provenance};
use crate::worktree::WorktreeInfo;
use anyhow::{Result, bail};

pub(crate) async fn prepare(info: &WorktreeInfo, task_id: &str) -> Result<()> {
    let status = git_process::run(&info.path, &["status", "--porcelain"]).await?;
    if !status.status.success() {
        bail!(
            "git status failed: {}",
            String::from_utf8_lossy(&status.stderr)
        );
    }
    if String::from_utf8_lossy(&status.stdout).trim().is_empty() {
        return Ok(());
    }
    let add = git_process::run(&info.path, &["add", "--all"]).await?;
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
#[path = "worktree_commit_tests.rs"]
mod tests;
