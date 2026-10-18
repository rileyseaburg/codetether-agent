//! Forage scan reporting.
use super::*;
impl RunEvents<'_> {
    /// Publish that the cycle is scanning for opportunities.
    pub fn scan(&self, cycle_task_id: &str) {
        let _ = self.bus_handle.send_task_update(
            cycle_task_id,
            TaskState::Working,
            Some("scanning OKR opportunities".to_string()),
        );
    }
}
