//! Record the final unsuccessful quality attempt.
use super::parallel_execution_context::ParallelExecutionContext;
use super::parallel_quality_state::QualityProgress;
use super::*;
impl ParallelExecutionContext<'_> {
    pub(super) fn quality_exhausted(
        &self,
        state: &mut QualityProgress,
        error_output: &str,
        max_quality_retries: usize,
    ) {
        state.remember_failure(error_output);
        warn!(
            story_id = %self.story.id,
            "Quality gates still failing after {} retries",
            max_quality_retries
        );
    }
}
