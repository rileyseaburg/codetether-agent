//! Swarm agent activity event reduction.
use crate::{SwarmEvent, SwarmViewState};
impl SwarmViewState {
    pub(crate) fn apply_agent_activity(&mut self, event: SwarmEvent) {
        match event {
            SwarmEvent::AgentToolCall {
                subtask_id,
                tool_name,
            } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.current_tool = Some(tool_name);
                    task.steps += 1;
                }
            }
            SwarmEvent::AgentToolCallDetail { subtask_id, detail } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.current_tool = None;
                    codetether_view_retention::agent_update::push(
                        &mut task.tool_call_history,
                        detail,
                    );
                }
            }
            SwarmEvent::AgentMessage { subtask_id, entry } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    codetether_view_retention::agent_update::push(&mut task.messages, entry);
                }
            }
            _ => unreachable!("event routed to incorrect swarm reducer"),
        }
    }
}
