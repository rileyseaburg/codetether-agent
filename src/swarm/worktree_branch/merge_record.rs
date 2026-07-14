//! Record a swarm worktree merge without hiding failed artifacts.

use super::SubTaskResult;
use crate::worktree::{MergeResult, WorktreeInfo, WorktreeManager};

#[path = "merge_failure.rs"]
mod failure;
pub(super) use failure::fail;

pub(super) async fn apply(
    result: &mut SubTaskResult,
    manager: &WorktreeManager,
    worktree: &WorktreeInfo,
    merged: anyhow::Result<MergeResult>,
) {
    match merged {
        Ok(merged) if merged.success => success(result, manager, worktree, merged).await,
        Ok(merged) => failure::merge(result, &merged),
        Err(error) => fail(result, format!("Failed to merge worktree: {error}")),
    }
}

async fn success(
    result: &mut SubTaskResult,
    manager: &WorktreeManager,
    worktree: &WorktreeInfo,
    merged: MergeResult,
) {
    tracing::info!(subtask_id = %result.subtask_id, files = merged.files_changed,
        "Merged swarm worktree changes");
    result
        .result
        .push_str(&format!("\n\n--- Merge Result ---\n{}", merged.summary));
    if let Err(error) = manager.cleanup(&worktree.name).await {
        tracing::warn!(%error, "Failed to cleanup merged swarm worktree");
    }
}

#[cfg(test)]
#[path = "merge_outcome_tests.rs"]
mod tests;
