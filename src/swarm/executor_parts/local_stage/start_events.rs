//! Events emitted when a local sub-agent starts.

use super::job::Job;
use crate::swarm::SubTaskStatus;
use crate::tui::swarm_view::SwarmEvent;

pub(super) fn emit(job: &Job) {
    let Some(events) = &job.events else { return };
    let agent = format!("agent-{}", job.id);
    let _ = events.try_send(SwarmEvent::SubTaskUpdate {
        id: job.id.clone(),
        name: job.name.clone(),
        status: SubTaskStatus::Running,
        agent_name: Some(agent.clone()),
    });
    let _ = events.try_send(SwarmEvent::AgentStarted {
        subtask_id: job.id.clone(),
        agent_name: agent,
        specialty: job.specialty.clone(),
    });
}
