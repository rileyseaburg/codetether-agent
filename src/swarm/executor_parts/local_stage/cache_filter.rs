//! Cache short-circuiting before local agent launch.

use super::super::cache_result;
use super::state::State;
use crate::swarm::{SubTask, SubTaskStatus};
use crate::tui::swarm_view::SwarmEvent;

pub(super) async fn pending(state: &mut State<'_>, tasks: Vec<SubTask>) -> Vec<SubTask> {
    let mut pending = Vec::with_capacity(tasks.len());
    for task in tasks {
        let cached = if task.is_read_only() {
            match &state.executor.cache {
                Some(cache) => cache.lock().await.get(&task).await,
                None => None,
            }
        } else {
            None
        };
        if let Some(result) = cached {
            tracing::info!(subtask_id = %task.id, "Using cached subtask result");
            state.executor.try_send_event(SwarmEvent::SubTaskUpdate {
                id: task.id.clone(),
                name: task.name.clone(),
                status: SubTaskStatus::Completed,
                agent_name: Some("cached".into()),
            });
            state
                .immediate_results
                .push(cache_result::retarget(result, &task));
        } else {
            pending.push(task);
        }
    }
    pending
}
