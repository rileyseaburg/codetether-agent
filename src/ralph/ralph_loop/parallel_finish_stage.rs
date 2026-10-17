//! Increment once, publish stage learnings, and persist the updated PRD.
use super::*;
impl RalphLoop {
    pub(super) async fn finish_parallel_stage(&mut self) -> anyhow::Result<()> {
        // Increment iteration once per stage, not per story
        self.state.current_iteration += 1;

        // Publish stage-level progress to bus after all stories in this stage
        if self.bus.is_some() {
            let bus_learnings: Vec<String> = self
                .state
                .progress_log
                .iter()
                .flat_map(|e| e.learnings.clone())
                .collect();
            for story in &self.state.prd.user_stories {
                if story.passes {
                    self.bus_publish_story_result(
                        story,
                        self.state.current_iteration,
                        &bus_learnings,
                        None,
                    );
                }
            }
        }

        // Save PRD after each stage
        self.state.prd.save(&self.state.prd_path).await?;

        // Persist PRD + iteration to store after each stage
        codetether_ralph_core::state_store::updates::checkpoint(
            self.store.as_ref(),
            &self.run_id,
            &self.state,
        );
        Ok(())
    }
}
