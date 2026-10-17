//! Iterate quality checks, verification and targeted repair attempts.
use super::parallel_execution_context::ParallelExecutionContext;
use super::parallel_quality_state::QualityProgress;
use super::*;
impl ParallelExecutionContext<'_> {
    pub(super) async fn run_quality_retries(
        &self,
        state: &mut QualityProgress,
        quality_checks: &QualityChecks,
        max_quality_retries: usize,
    ) {
        for qg_attempt in 0..=max_quality_retries {
            let (all_ok, mut error_output) = codetether_ralph_core::quality::run_for_repair(
                self.working_dir,
                quality_checks,
                &quality::QualityHost,
            )
            .await;
            if all_ok {
                if self
                    .verify_quality_attempt(state, &mut error_output, qg_attempt)
                    .await
                {
                    break;
                }
            }

            if qg_attempt < max_quality_retries {
                if !self
                    .run_quality_repair(state, &error_output, qg_attempt, max_quality_retries)
                    .await
                {
                    break;
                }
            } else {
                self.quality_exhausted(state, &error_output, max_quality_retries);
            }
        }
    }
}
