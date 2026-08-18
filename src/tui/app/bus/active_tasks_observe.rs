//! Bounded mutation of protocol-level active tasks.

use std::time::Instant;

use crate::bus::BusMessage;

use super::ActiveTasks;

pub(super) const MAX_ACTIVE_TASKS: usize = 1_024;

impl ActiveTasks {
    /// Update tracking from a bus message. No-op for non-task messages.
    pub fn observe(&mut self, message: &BusMessage) {
        let BusMessage::TaskUpdate { task_id, state, .. } = message else {
            return;
        };
        if state.is_terminal() {
            self.ids.remove(task_id);
            return;
        }
        if self.ids.contains_key(task_id) {
            return;
        }
        if self.ids.len() >= MAX_ACTIVE_TASKS
            && let Some(oldest) = self
                .ids
                .iter()
                .min_by_key(|(_, started)| *started)
                .map(|(id, _)| id.clone())
        {
            self.ids.remove(&oldest);
        }
        self.ids.insert(task_id.clone(), Instant::now());
    }
}
