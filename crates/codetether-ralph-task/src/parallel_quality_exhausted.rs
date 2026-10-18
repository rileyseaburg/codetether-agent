//! Record the final unsuccessful quality attempt.
use super::parallel_execution_context::ParallelExecutionContext;
use super::*;
impl<H: Host> ParallelExecutionContext<'_, H> {
    pub(crate) fn quality_exhausted(
        &self,
        state: &mut QualityProgress,
        error_output: &str,
        max_quality_retries: usize,
    ) {
        state.remember_failure(error_output);
        warn!(target: "codetether_agent::ralph::ralph_loop::parallel_quality_exhausted",
            story_id = %self.story.id,
            "Quality gates still failing after {} retries",
            max_quality_retries
        );
    }
}
