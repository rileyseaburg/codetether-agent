//! Lifecycle events for Kubernetes-backed subtasks.

use super::state::State;
use crate::swarm::{SubTask, SubTaskResult, SubTaskStatus};
use crate::tui::swarm_view::SwarmEvent;

pub(super) fn started(state: &State<'_>, task: &SubTask, branch: &str) {
    state.executor.try_send_event(SwarmEvent::SubTaskUpdate {
        id: task.id.clone(),
        name: task.name.clone(),
        status: SubTaskStatus::Running,
        agent_name: Some(format!("k8s-{branch}")),
    });
    state.executor.try_send_event(SwarmEvent::AgentStarted {
        subtask_id: task.id.clone(),
        agent_name: format!("k8s-{branch}"),
        specialty: task
            .specialty
            .clone()
            .unwrap_or_else(|| "generalist".into()),
    });
}

pub(super) fn failed(state: &State<'_>, task: &SubTask, agent: &str, error: &str) {
    state.executor.try_send_event(SwarmEvent::SubTaskUpdate {
        id: task.id.clone(),
        name: task.name.clone(),
        status: SubTaskStatus::Failed,
        agent_name: Some(agent.into()),
    });
    state.executor.try_send_event(SwarmEvent::AgentError {
        subtask_id: task.id.clone(),
        error: error.into(),
    });
}

pub(super) fn finished(state: &State<'_>, result: &SubTaskResult) {
    super::finish_events::emit(state, result);
}
