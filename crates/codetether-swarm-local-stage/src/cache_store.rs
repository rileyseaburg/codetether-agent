//! Storage of successful read-only local subtask results.

use super::state::State;
use codetether_swarm_core::SubTaskResult;

pub(super) async fn put(state: &State<'_, impl crate::Host>, result: &SubTaskResult) {
    if !result.success {
        return;
    }
    let Some(task) = state.cache_tasks.get(&result.subtask_id) else {
        return;
    };
    if let Err(error) = state.executor.cache_put(task, result).await {
        tracing::warn!(subtask_id = %result.subtask_id, %error, "Failed to cache subtask result");
    }
}
