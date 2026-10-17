//! Forage sleeping reporting.
use super::*;
impl RunEvents<'_> {
    pub(in crate::forage) fn sleeping(
        &self,
        cycle: usize,
        interval_secs: u64,
        cycle_task_id: &str,
    ) {
        tracing::info!(target: "codetether_agent::forage",
            cycle,
            interval_secs,
            "Forage loop sleeping before next cycle"
        );
        let _ = self.bus_handle.send_task_update(
            cycle_task_id,
            TaskState::Submitted,
            Some(format!("sleeping {interval_secs}s before next cycle")),
        );
    }
}
