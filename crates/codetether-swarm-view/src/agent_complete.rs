//! Swarm agent complete event reduction.
use crate::{SwarmEvent, SwarmViewState};
use codetether_swarm_core::SubTaskStatus;
impl SwarmViewState {
    pub(crate) fn apply_agent_complete(&mut self, event: SwarmEvent) {
        match event {
            SwarmEvent::AgentComplete {
                subtask_id,
                success,
                steps,
            } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.status = if success {
                        SubTaskStatus::Completed
                    } else {
                        SubTaskStatus::Failed
                    };
                    task.steps = steps;
                    task.current_tool = None;
                    if let Some(start) = task.started_at {
                        task.elapsed_secs = Some(start.elapsed().as_secs());
                    }
                }
            }
            _ => unreachable!("event routed to incorrect swarm reducer"),
        }
    }
}
