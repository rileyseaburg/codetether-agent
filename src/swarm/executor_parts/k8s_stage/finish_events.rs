//! Result events emitted for a finished Kubernetes pod.

use super::state::State;
use crate::swarm::{SubTaskResult, SubTaskStatus};
use crate::tui::swarm_view::SwarmEvent;

pub(super) fn emit(state: &State<'_>, result: &SubTaskResult) {
    let name = state
        .names
        .get(&result.subtask_id)
        .cloned()
        .unwrap_or_else(|| result.subtask_id.clone());
    let status = if result.success {
        SubTaskStatus::Completed
    } else {
        SubTaskStatus::Failed
    };
    state.executor.try_send_event(SwarmEvent::SubTaskUpdate {
        id: result.subtask_id.clone(),
        name,
        status,
        agent_name: Some(format!("k8s-{}", result.subtask_id)),
    });
    if let Some(error) = &result.error {
        state.executor.try_send_event(SwarmEvent::AgentError {
            subtask_id: result.subtask_id.clone(),
            error: error.clone(),
        });
    }
    state.executor.try_send_event(SwarmEvent::AgentOutput {
        subtask_id: result.subtask_id.clone(),
        output: result.result.clone(),
    });
    state.executor.try_send_event(SwarmEvent::AgentComplete {
        subtask_id: result.subtask_id.clone(),
        success: result.success,
        steps: result.steps,
    });
}
