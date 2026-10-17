//! Stop completed/exhausted runs before announcing the next stage.
use super::*;
impl RalphLoop {
    pub(super) fn begin_parallel_stage(
        &mut self,
        stage_idx: usize,
        total_stages: usize,
        stage_stories: &[UserStory],
    ) -> bool {
        if self.state.prd.is_complete() {
            info!("All stories complete!");
            self.state.status = RalphStatus::Completed;
            return false;
        }

        if self.state.current_iteration >= self.state.max_iterations {
            return false;
        }

        let story_count = stage_stories.len();
        info!(
            "=== Stage {}/{}: {} stories in parallel ===",
            stage_idx + 1,
            total_stages,
            story_count
        );

        true
    }
}
