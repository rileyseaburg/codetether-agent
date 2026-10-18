//! Verify a parallel story with or without configured quality commands.
use super::parallel_execution_context::ParallelExecutionContext;
use super::*;
impl<H: Host> ParallelExecutionContext<'_, H> {
    pub(crate) async fn verify_quality_attempt(
        &self,
        state: &mut QualityProgress,
        error_output: &mut String,
        qg_attempt: usize,
    ) -> bool {
        codetether_ralph_core::quality_repair::verify_attempt(
            state,
            error_output,
            H::verify(self.working_dir, self.story),
            || {
                info!(target: "codetether_agent::ralph::ralph_loop::parallel_quality_verify",
                    story_id = %self.story.id,
                    attempt = qg_attempt + 1,
                    "Quality gates and verification passed"
                );
            },
        )
        .await
    }
    pub(crate) async fn verify_without_quality(&self, state: &mut QualityProgress) {
        codetether_ralph_core::quality_repair::verify_without_checks(
            state,
            H::verify(self.working_dir, self.story),
        )
        .await;
    }
}
