//! Remote subtask payload creation for Kubernetes pods.

use super::{context, state::State};
use codetether_swarm_core::SubTask;
use codetether_swarm_core::remote_protocol::RemoteSubtaskPayload;

pub(super) async fn build(
    state: &State<'_, impl crate::Host>,
    task: &SubTask,
) -> RemoteSubtaskPayload {
    let completed = state.completed.read().await;
    let context = context::build(task, &completed);
    RemoteSubtaskPayload {
        swarm_id: state.swarm_id.into(),
        subtask_id: task.id.clone(),
        subtask_name: task.name.clone(),
        specialty: task.specialty.clone().unwrap_or_default(),
        instruction: task.instruction.clone(),
        context,
        provider: state.provider.clone(),
        model: state.model.clone(),
        max_steps: state.executor.config().max_steps_per_subagent,
        timeout_secs: state.executor.config().subagent_timeout_secs,
        working_dir: state.executor.config().working_dir.clone(),
        read_only: task.is_read_only(),
        verification: task.is_verification(),
        probe_interval_secs: u64::from(state.executor.config().collapse_enabled) * 5,
    }
}
