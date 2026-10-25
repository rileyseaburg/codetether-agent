//! Delivery outcome handling for an isolated direct-swarm worker.
//!
//! Successful work is applied to the checkout the swarm was spawned from as
//! uncommitted edits. Nothing is committed or merged into the user's branch.

use super::state::SwarmWorktrees;
use crate::task_result::TaskResult;
use codetether_worktree::WorktreeInfo;

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
    let delivery = match worktrees.mgr.deliver(&worktree.name).await {
        Ok(delivery) => delivery,
        Err(failure) => return error(result, failure),
    };
    let summary = delivery.summary();
    if !delivery.status.is_delivered() {
        result.success = false;
        result.error = Some(summary.clone());
        result
            .output
            .push_str(&format!("\n\n--- Delivery Failed ---\n{summary}"));
        return;
    }
    result
        .output
        .push_str(&format!("\n\n--- Delivery Result ---\n{summary}"));
    if let Err(error) = worktrees.mgr.discard_delivered(&worktree.name).await {
        tracing::warn!(%error, "Failed to cleanup delivered swarm worktree");
    }
}

pub(super) fn error(result: &mut TaskResult, failure: anyhow::Error) {
    tracing::error!(error = %failure, "Failed to deliver swarm worktree");
    result.success = false;
    result.error = Some(format!("worktree delivery failed: {failure}"));
}
