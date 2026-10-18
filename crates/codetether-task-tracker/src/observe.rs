//! Bounded mutation of protocol-level active tasks.

use std::time::Instant;

use codetether_bus_core::BusMessage;

use super::ActiveTasks;

pub(super) const MAX_ACTIVE_TASKS: usize = 1_024;

impl ActiveTasks {
    /// Update tracking from a bus message. No-op for non-task messages.
    ///
    /// # Arguments
    /// * `message` — Bus event whose task state may update the active set.
    ///
    /// # Returns
    /// Updates tracking in place, retaining at most 1,024 active task IDs.
    ///
    /// # Examples
    /// ```
    /// let mut tasks = codetether_task_tracker::ActiveTasks::default();
    /// tasks.observe(&codetether_bus_core::BusMessage::AgentShutdown { agent_id: "peer".into() });
    /// assert_eq!(tasks.count(), 0);
    /// ```
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
