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
        match run_story_verification(self.working_dir, self.story).await {
            Ok(()) => {
                state.qg_passed = true;
                state.failure_reason = None;
                info!(
                    story_id = %self.story.id,
                    attempt = qg_attempt + 1,
                    "Quality gates and verification passed"
                );
                return true;
            }
            Err(e) => {
                *error_output = format!("Verification steps failed:\n{e}");
                state.failure_reason = Some(error_output.clone());
            }
        }
        false
    }
    pub(super) async fn verify_without_quality(&self, state: &mut QualityProgress) {
        match run_story_verification(self.working_dir, self.story).await {
            Ok(()) => {
                state.qg_passed = true;
                state.failure_reason = None;
            }
            Err(e) => {
                state.failure_reason = Some(format!("Verification steps failed:\n{e}"));
            }
        }
    }
}
