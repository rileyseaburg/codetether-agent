//! Verify a parallel story with or without configured quality commands.
use super::parallel_execution_context::ParallelExecutionContext;
use super::parallel_quality_state::QualityProgress;
use super::*;
impl ParallelExecutionContext<'_> {
    pub(super) async fn verify_quality_attempt(
        &self,
        state: &mut QualityProgress,
        error_output: &mut String,
        qg_attempt: usize,
    ) -> bool {
        codetether_ralph_core::quality_repair::verify_attempt(
            state,
            error_output,
            run_story_verification(self.working_dir, self.story),
            || {
                info!(
                    story_id = %self.story.id,
                    attempt = qg_attempt + 1,
                    "Quality gates and verification passed"
                );
            },
        )
        .await
    }
    pub(super) async fn verify_without_quality(&self, state: &mut QualityProgress) {
        codetether_ralph_core::quality_repair::verify_without_checks(
            state,
            run_story_verification(self.working_dir, self.story),
        )
        .await;
    }
}
