//! Swarm agent results event reduction.
use crate::{SwarmEvent, SwarmViewState};
impl SwarmViewState {
    pub(crate) fn apply_agent_results(&mut self, event: SwarmEvent) {
        match event {
            SwarmEvent::AgentOutput { subtask_id, output } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    codetether_view_retention::agent_update::output(
                        &mut task.output,
                        &output,
                        "swarm output",
                    );
                }
            }
            SwarmEvent::AgentError { subtask_id, error } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.error = Some(error);
                }
            }
            _ => unreachable!("event routed to incorrect swarm reducer"),
        }
    }
}
