//! Cache short-circuiting before local agent launch.

use super::state::State;
use codetether_swarm_core::SwarmEvent;
use codetether_swarm_core::cache as cache_result;
use codetether_swarm_core::{SubTask, SubTaskStatus};

pub(super) async fn pending(
    state: &mut State<'_, impl crate::Host>,
    tasks: Vec<SubTask>,
) -> Vec<SubTask> {
    let mut pending = Vec::with_capacity(tasks.len());
    for task in tasks {
        let cached = if task.is_read_only() {
            state.executor.cache_get(&task).await
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
