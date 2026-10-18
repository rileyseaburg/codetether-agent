//! Swarm agent start event reduction.
use crate::{SwarmEvent, SwarmViewState};
use codetether_swarm_core::SubTaskStatus;
impl SwarmViewState {
    pub(crate) fn apply_agent_start(&mut self, event: SwarmEvent) {
        match event {
            SwarmEvent::SubTaskUpdate {
                id,
                name,
                status,
                agent_name,
            } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == id) {
                    task.status = status;
                    task.name = name;
                    if agent_name.is_some() {
                        task.agent_name = agent_name;
                    }
                }
            }
            SwarmEvent::AgentStarted {
                subtask_id,
                agent_name,
                ..
            } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.status = SubTaskStatus::Running;
                    task.agent_name = Some(agent_name);
                    task.started_at = Some(std::time::Instant::now());
                }
            }
            _ => unreachable!("event routed to incorrect swarm reducer"),
        }
    }
}
