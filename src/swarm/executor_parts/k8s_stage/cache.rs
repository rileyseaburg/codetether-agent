//! Kubernetes-stage cache lookup and storage.

use super::super::cache_result;
use super::state::State;
use crate::swarm::{SubTask, SubTaskResult, SubTaskStatus};
use crate::tui::swarm_view::SwarmEvent;

pub(super) async fn find(state: &State<'_>, task: &SubTask) -> Option<SubTaskResult> {
    if !task.is_read_only() {
        return None;
    }
    let cache = state.executor.cache.as_ref()?;
    let result = cache.lock().await.get(task).await?;
    state.executor.try_send_event(SwarmEvent::SubTaskUpdate {
        id: task.id.clone(),
        name: task.name.clone(),
        status: SubTaskStatus::Completed,
        agent_name: Some("cached".into()),
    });
    Some(cache_result::retarget(result, task))
}

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
        tracing::warn!(subtask_id = %result.subtask_id, %error, "Failed to cache K8s result");
    }
}
