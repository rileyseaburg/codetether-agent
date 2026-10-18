//! Forage complete reporting.
use super::*;
impl RunEvents<'_> {
    /// Mark the completed cycle with its continuation decision.
    pub fn complete(&self, cycle_task_id: &str, keep_running: bool) {
        let _ = self.bus_handle.send_task_update(
            cycle_task_id,
            TaskState::Completed,
            Some(format!("cycle complete (keep_running={keep_running})")),
        );
    }
}
