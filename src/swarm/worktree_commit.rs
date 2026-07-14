//! Commit dirty swarm worktree edits before branch integration.

use crate::provenance::{ExecutionOrigin, ExecutionProvenance, git_commit_with_provenance};
use crate::worktree::WorktreeInfo;
use anyhow::{Context, Result, bail};
use tokio::process::Command;

pub(crate) async fn prepare(info: &WorktreeInfo, task_id: &str) -> Result<()> {
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

async fn git(path: &std::path::Path, args: &[&str]) -> Result<std::process::Output> {
    Command::new("git")
        .args(args)
        .current_dir(path)
        .output()
        .await
        .with_context(|| format!("failed to run git {}", args.join(" ")))
}

#[cfg(test)]
#[path = "worktree_commit_tests.rs"]
mod tests;
