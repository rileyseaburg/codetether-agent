//! Kubernetes-stage cache lookup and storage.

use super::state::State;
use crate::Cache;
use codetether_swarm_core::SwarmEvent;
use codetether_swarm_core::cache as cache_result;
use codetether_swarm_core::{SubTask, SubTaskResult, SubTaskStatus};

pub(super) async fn find(
    state: &State<'_, impl crate::Host>,
    task: &SubTask,
) -> Option<SubTaskResult> {
    if !task.is_read_only() {
        return None;
    }
    let cache = state.executor.cache()?;
    let result = cache.lock().await.get(task).await?;
    state.executor.try_send_event(SwarmEvent::SubTaskUpdate {
        id: task.id.clone(),
        name: task.name.clone(),
        status: SubTaskStatus::Completed,
        agent_name: Some("cached".into()),
    });
    Some(cache_result::retarget(result, task))
}

pub(super) async fn put(state: &State<'_, impl crate::Host>, result: &SubTaskResult) {
    if !result.success {
        return;
    }
    let (Some(cache), Some(task)) = (
        state.executor.cache(),
        state.cache_tasks.get(&result.subtask_id),
    ) else {
        return;
    };
    if let Err(error) = cache.lock().await.put(task, result).await {
        tracing::warn!(subtask_id = %result.subtask_id, %error, "Failed to cache K8s result");
    }
}
