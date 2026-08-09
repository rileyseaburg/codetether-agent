//! Integration policy for one completed local worktree.

use super::super::merge_outcome;
use super::{killed_worktree, state::State};
use crate::swarm::SubTaskResult;
use crate::worktree::WorktreeInfo;

pub(super) async fn apply(state: &State<'_>, result: &mut SubTaskResult, worktree: &WorktreeInfo) {
    record_artifacts(result, worktree).await;
    if let Some(reason) = state.kill_reasons.get(&result.subtask_id) {
        return killed_worktree::apply(state, result, worktree, reason).await;
    }
    if result.success && state.executor.config.worktree_auto_merge {
        if let Some(manager) = &state.manager {
            let expects = state
                .change_expectations
                .get(&result.subtask_id)
                .copied()
                .unwrap_or(true);
            merge_outcome::apply(result, manager, worktree, expects).await;
        }
    } else if !result.success {
        report_unverified_work(result, worktree);
    }
}

/// Records files the sub-agent wrote, independent of its self-report.
async fn record_artifacts(result: &mut SubTaskResult, worktree: &WorktreeInfo) {
    if result.artifacts.is_empty() {
        result.artifacts =
            crate::swarm::worktree_commit::inventory::written_files(&worktree.path).await;
    }
}

/// Surfaces work that exists on disk even though the agent reported failure.
///
/// A child that wrote files and then failed *verification* reports a blocker,
/// so the orchestrator must see the artifact count to avoid discarding or
/// rebuilding delivered work.
fn report_unverified_work(result: &SubTaskResult, worktree: &WorktreeInfo) {
    tracing::info!(
        subtask_id = %result.subtask_id,
        path = %worktree.path.display(),
        files_written = result.artifacts.len(),
        "Keeping failed subtask worktree; agent reported failure but wrote files"
    );
}
