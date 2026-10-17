//! Turn an initial model result into a verified or failed task outcome.
use super::parallel_execution_context::ParallelExecutionContext;
use super::parallel_quality_state::QualityProgress;
use super::*;
impl ParallelExecutionContext<'_> {
    pub(super) async fn finish_quality(
        &self,
        call_result: anyhow::Result<String>,
        quality_checks_enabled: bool,
        quality_checks: &QualityChecks,
        max_quality_retries: usize,
    ) -> (anyhow::Result<String>, bool, Option<String>) {
        match call_result {
            Ok(initial_output) => {
                let mut state = QualityProgress {
                    output: initial_output,
                    qg_passed: false,
                    failure_reason: None,
                };
                if quality_checks_enabled {
                    self.run_quality_retries(&mut state, quality_checks, max_quality_retries)
                        .await;
                } else {
                    self.verify_without_quality(&mut state).await;
                }
                (Ok(state.output), state.qg_passed, state.failure_reason)
            }
            Err(e) => (Err(e), false, None),
        }
    }
}
