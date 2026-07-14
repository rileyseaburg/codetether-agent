//! Storage of successful read-only local subtask results.

use super::state::State;
use crate::swarm::SubTaskResult;

pub(super) async fn put(state: &State<'_>, result: &SubTaskResult) {
    if !result.success {
        return;
    }
    let (Some(cache), Some(task)) = (
        state.executor.cache.as_ref(),
        state.cache_tasks.get(&result.subtask_id),
    ) else {
        return;
    };
    if let Err(error) = cache.lock().await.put(task, result).await {
        tracing::warn!(subtask_id = %result.subtask_id, %error, "Failed to cache subtask result");
    }
}
