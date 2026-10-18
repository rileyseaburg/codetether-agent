//! Ralph view progress.
use crate::{RalphStoryStatus, RalphViewState};
impl RalphViewState {
    /// Returns pending, running, passed and failed counts, in that order.
    pub fn status_counts(&self) -> (usize, usize, usize, usize) {
        let mut pending = 0;
        let mut running = 0;
        let mut passed = 0;
        let mut failed = 0;
        for story in &self.stories {
            match story.status {
                RalphStoryStatus::Pending | RalphStoryStatus::Blocked => pending += 1,
                RalphStoryStatus::Running | RalphStoryStatus::QualityCheck => running += 1,
                RalphStoryStatus::Passed => passed += 1,
                RalphStoryStatus::Failed => failed += 1,
            }
        }
        (pending, running, passed, failed)
    }

    /// Overall progress as percentage
    pub fn progress(&self) -> f64 {
        if self.stories.is_empty() {
            return 0.0;
        }
        let (_, _, passed, failed) = self.status_counts();
        ((passed + failed) as f64 / self.stories.len() as f64) * 100.0
    }
}
