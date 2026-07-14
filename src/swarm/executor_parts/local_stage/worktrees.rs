//! Concurrent worktree provisioning for mutating subtasks.

use super::super::{worktree_failure, worktree_setup};
use super::state::State;
use crate::swarm::SubTask;
use std::sync::Arc;

pub(super) async fn prepare(state: &mut State<'_>, pending: Vec<SubTask>) -> Vec<SubTask> {
    let Some(manager) = &state.manager else {
        return pending;
    };
    let ids = pending
        .iter()
        .filter(|task| task.needs_worktree())
        .map(|task| task.id.clone())
        .collect();
    state.ready_worktrees = worktree_setup::precreate(Arc::clone(manager), ids).await;
    for (id, worktree) in &state.ready_worktrees {
        state.active_worktrees.insert(id.clone(), worktree.clone());
        state.all_worktrees.insert(id.clone(), worktree.clone());
    }
    pending
        .into_iter()
        .filter(|task| {
            let ready = !task.needs_worktree() || state.ready_worktrees.contains_key(&task.id);
            if !ready {
                state
                    .immediate_results
                    .push(worktree_failure::required(task));
            }
            ready
        })
        .collect()
}
