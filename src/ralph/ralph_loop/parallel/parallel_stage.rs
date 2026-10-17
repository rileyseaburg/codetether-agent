//! Stop completed/exhausted runs before announcing the next stage.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn begin_parallel_stage(
        &mut self,
        stage_idx: usize,
        total_stages: usize,
        stage_stories: &[UserStory],
    ) -> bool {
        if self.state.prd.is_complete() {
            info!(target: "codetether_agent::ralph::ralph_loop::parallel_stage", "All stories complete!");
            self.state.status = RalphStatus::Completed;
            return false;
        }

        if self.state.current_iteration >= self.state.max_iterations {
            return false;
        }

        let story_count = stage_stories.len();
        info!(target: "codetether_agent::ralph::ralph_loop::parallel_stage",
            "=== Stage {}/{}: {} stories in parallel ===",
            stage_idx + 1,
            total_stages,
            story_count
        );

        true
    }
}
