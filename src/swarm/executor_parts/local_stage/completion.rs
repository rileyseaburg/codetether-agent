//! Collection of one completed local task handle.

use super::{delegation_outcome, failure, state::State};

pub(super) fn record(
    state: &mut State<'_>,
    joined: Result<(String, anyhow::Result<crate::swarm::SubTaskResult>), tokio::task::JoinError>,
) {
    let (id, result) = match joined {
        Ok((id, Ok(result))) => (id, result),
        Ok((id, Err(error))) => {
            let result = failure::result(id.clone(), error.to_string());
            (id, result)
        }
        Err(error) => {
            let id = state
                .task_ids
                .remove(&error.id())
                .unwrap_or_else(|| "unknown".into());
            let result = failure::result(id.clone(), format!("Task join error: {error}"));
            (id, result)
        }
    };
    state.aborts.remove(&id);
    state.active_worktrees.remove(&id);
    let worktree = state.all_worktrees.get(&id).cloned();
    delegation_outcome::record(state, &id, result.success);
    state.completed.push((result, worktree));
}
