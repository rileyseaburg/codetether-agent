//! Merge outcome handling for an isolated direct-swarm worker.

use super::super::TaskResult;
use super::state::SwarmWorktrees;
use crate::worktree::WorktreeInfo;

pub(super) async fn apply(
    worktrees: &SwarmWorktrees,
    index: usize,
    result: &mut TaskResult,
    worktree: &WorktreeInfo,
    contributed: bool,
) {
    match super::premerge::apply(worktrees, index, result, worktree, contributed).await {
        Ok(true) => {}
        Ok(false) => return,
        Err(error) => return self::error(result, error),
    }
    match worktrees.mgr.merge(&worktree.name).await {
        Ok(merge) if merge.success => {
            result
                .output
                .push_str(&format!("\n\n--- Merge Result ---\n{}", merge.summary));
            if let Err(error) = worktrees.mgr.cleanup(&worktree.name).await {
                tracing::warn!(%error, "Failed to cleanup swarm worktree");
            }
        }
        Ok(merge) => {
            result.success = false;
            result.error = Some(merge.summary.clone());
            result
                .output
                .push_str(&format!("\n\n--- Merge Failed ---\n{}", merge.summary));
        }
        Err(failure) => error(result, failure),
    }
}

pub(super) fn error(result: &mut TaskResult, failure: anyhow::Error) {
    tracing::error!(error = %failure, "Failed to merge swarm worktree");
    result.success = false;
    result.error = Some(format!("worktree merge failed: {failure}"));
}
