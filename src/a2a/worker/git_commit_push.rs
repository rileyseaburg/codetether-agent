//! Commit and push worker task changes.

use super::{git_commit_push_ops as ops, task_timeline};
use anyhow::Result;
use std::path::Path;

pub(super) async fn run(
    repo_path: &Path,
    task_id: &str,
    provenance: Option<&crate::provenance::ExecutionProvenance>,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<Option<String>> {
    if !repo_path.join(".git").exists() || ops::status(repo_path).await?.trim().is_empty() {
        return Ok(None);
    }
    timeline.checkpoint(task_timeline::TaskCheckpoint::CommitStaging);
    ops::git(repo_path, &["add", "--all"]).await?;
    if ops::status(repo_path).await?.trim().is_empty() {
        return Ok(None);
    }
    ops::commit(repo_path, task_id, provenance).await?;
    timeline.checkpoint(task_timeline::TaskCheckpoint::CommitCreated);
    let commit_sha = ops::git(repo_path, &["rev-parse", "HEAD"])
        .await
        .unwrap_or_default();
    let branch = ops::git(repo_path, &["rev-parse", "--abbrev-ref", "HEAD"])
        .await
        .unwrap_or_else(|_| "HEAD".to_string());
    timeline.checkpoint(task_timeline::TaskCheckpoint::CommitPushing);
    ops::push(repo_path, branch.trim()).await?;
    timeline.checkpoint(task_timeline::TaskCheckpoint::CommitPushed);
    Ok(Some(format!(
        "Committed and pushed task changes: {} on {}",
        commit_sha.trim(),
        branch.trim()
    )))
}
