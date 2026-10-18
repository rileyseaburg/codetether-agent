//! Swarm view lifecycle.
use crate::SwarmEvent;
use crate::SwarmViewState;
impl SwarmViewState {
    /// Creates an inactive swarm view with no event receiver.
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies a run-start event for the supplied task.
    pub fn mark_active(&mut self, task: impl Into<String>) {
        self.handle_event(SwarmEvent::Started {
            task: task.into(),
            total_subtasks: self.subtasks.len(),
        });
    }
}
