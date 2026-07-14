//! Deterministic integration of completed local subtask results.

use super::{cache_store, integrate_worktree, state::State};
use crate::swarm::SubTaskResult;

pub(super) async fn all(state: &mut State<'_>) -> Vec<SubTaskResult> {
    if let Some(promoted) = &state.promoted {
        state
            .completed
            .sort_by_key(|(result, _)| usize::from(&result.subtask_id != promoted));
    }
    let mut results = std::mem::take(&mut state.immediate_results);
    for (mut result, worktree) in std::mem::take(&mut state.completed) {
        if let Some(worktree) = worktree {
            integrate_worktree::apply(state, &mut result, &worktree).await;
        }
        cache_store::put(state, &result).await;
        results.push(result);
    }
    results
}
