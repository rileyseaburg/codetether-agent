//! Swarm view progress.
use crate::SwarmViewState;
use codetether_swarm_core::SubTaskStatus;
impl SwarmViewState {
    /// Returns pending, running, completed and failed task counts, in that order.
    pub fn status_counts(&self) -> (usize, usize, usize, usize) {
        let mut pending = 0;
        let mut running = 0;
        let mut completed = 0;
        let mut failed = 0;

        for task in &self.subtasks {
            match task.status {
                SubTaskStatus::Pending | SubTaskStatus::Blocked => pending += 1,
                SubTaskStatus::Running => running += 1,
                SubTaskStatus::Completed => completed += 1,
                SubTaskStatus::Failed | SubTaskStatus::Cancelled | SubTaskStatus::TimedOut => {
                    failed += 1
                }
            }
        }

        (pending, running, completed, failed)
    }

    /// Overall progress as percentage
    pub fn progress(&self) -> f64 {
        if self.subtasks.is_empty() {
            return 0.0;
        }
        let (_, _, completed, failed) = self.status_counts();
        ((completed + failed) as f64 / self.subtasks.len() as f64) * 100.0
    }
}
