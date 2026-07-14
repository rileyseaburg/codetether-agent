//! Integration policy for one completed local worktree.

use super::super::merge_outcome;
use super::{killed_worktree, state::State};
use crate::swarm::SubTaskResult;
use crate::worktree::WorktreeInfo;

pub(super) async fn apply(state: &State<'_>, result: &mut SubTaskResult, worktree: &WorktreeInfo) {
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
        tracing::info!(subtask_id = %result.subtask_id, path = %worktree.path.display(),
            "Keeping failed subtask worktree for debugging");
    }
}
