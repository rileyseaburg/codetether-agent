//! Protocol-level active-task tracking.
//!
//! Derives the set of *currently in-flight* A2A tasks from `TaskUpdate`
//! bus messages: a task is inserted when it enters a non-terminal state and
//! removed when it reaches a terminal one. This is the source of truth for
//! "how many agents are actually working right now" — distinct from the
//! static `worker_bridge_registered_agents` roster, which only grows as peers
//! announce themselves and never reflects live activity.

use std::collections::HashSet;

use crate::bus::BusMessage;

/// In-flight task IDs, maintained from `TaskUpdate` state transitions.
#[derive(Debug, Default)]
pub struct ActiveTasks {
    ids: HashSet<String>,
}

impl ActiveTasks {
    /// Update tracking from a bus message. No-op for non-`TaskUpdate` kinds.
    pub fn observe(&mut self, message: &BusMessage) {
        if let BusMessage::TaskUpdate { task_id, state, .. } = message {
            if state.is_terminal() {
                self.ids.remove(task_id);
            } else {
                self.ids.insert(task_id.clone());
            }
        }
    }

    /// Number of tasks currently in a non-terminal state.
    pub fn count(&self) -> usize {
        self.ids.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a::types::TaskState;

    fn update(task_id: &str, state: TaskState) -> BusMessage {
        BusMessage::TaskUpdate {
            task_id: task_id.to_string(),
            state,
            message: None,
        }
    }

    #[test]
    fn tracks_active_and_clears_on_terminal() {
        let mut t = ActiveTasks::default();
        t.observe(&update("a", TaskState::Working));
        t.observe(&update("b", TaskState::Submitted));
        assert_eq!(t.count(), 2);
        t.observe(&update("a", TaskState::Completed));
        assert_eq!(t.count(), 1);
        t.observe(&update("b", TaskState::Failed));
        assert_eq!(t.count(), 0);
    }

    #[test]
    fn ignores_non_task_messages() {
        let mut t = ActiveTasks::default();
        t.observe(&BusMessage::AgentShutdown {
            agent_id: "x".into(),
        });
        assert_eq!(t.count(), 0);
    }
}
