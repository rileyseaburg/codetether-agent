//! Sampling and application of local branch-collapse decisions.

use super::{collapse_audit, collapse_kill, state::State};
use crate::swarm::{BranchRuntimeState, CollapseController};

pub(super) async fn sample(state: &mut State<'_>, controller: &mut CollapseController) {
    let branches = state
        .active_worktrees
        .iter()
        .map(|(id, worktree)| BranchRuntimeState {
            subtask_id: id.clone(),
            branch: worktree.branch.clone(),
            worktree_path: worktree.path.clone(),
        })
        .collect::<Vec<_>>();
    let tick = match controller.sample(&branches) {
        Ok(tick) => tick,
        Err(error) => {
            tracing::warn!(%error, "Collapse controller sampling failed");
            return;
        }
    };
    if state.promoted != tick.promoted_subtask_id {
        state.promoted = tick.promoted_subtask_id;
        if let Some(id) = &state.promoted {
            tracing::info!(subtask_id = %id, "Collapse controller promoted branch");
            collapse_audit::promotion(state.swarm_id, id).await;
        }
    }
    for kill in tick.kills {
        collapse_kill::apply(state, kill).await;
    }
}
