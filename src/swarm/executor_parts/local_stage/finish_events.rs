//! Completion and failure events for a local sub-agent.

use super::job::Job;
use crate::swarm::{SubTaskResult, SubTaskStatus};
use crate::tui::swarm_view::SwarmEvent;

pub(super) fn emit(job: &Job, result: &SubTaskResult, status: SubTaskStatus) {
    let Some(events) = &job.events else { return };
    let _ = events.try_send(SwarmEvent::SubTaskUpdate {
        id: job.id.clone(),
        name: job.name.clone(),
        status,
        agent_name: Some(format!("agent-{}", job.id)),
    });
    if let Some(error) = &result.error {
        let _ = events.try_send(SwarmEvent::AgentError {
            subtask_id: job.id.clone(),
            error: error.clone(),
        });
    }
    let _ = events.try_send(SwarmEvent::AgentOutput {
        subtask_id: job.id.clone(),
        output: result.result.clone(),
    });
    let _ = events.try_send(SwarmEvent::AgentComplete {
        subtask_id: job.id.clone(),
        success: result.success,
        steps: result.steps,
    });
}
