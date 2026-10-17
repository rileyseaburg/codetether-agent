//! Run model retries and quality repair using this task's limits.
use super::parallel_execution_context::ParallelExecutionContext;
use super::parallel_task::ParallelTask;
use super::*;
impl ParallelTask {
    pub(in crate::ralph::ralph_loop) async fn execute(
        &self,
        execution: &ParallelExecutionContext<'_>,
        prompt: &str,
    ) -> (anyhow::Result<String>, bool, Option<String>) {
        let call_result = codetether_ralph_core::rate_limit::retry(
            self.max_rate_limit_retries,
            self.rate_limit_base_delay_ms,
            &self.story.id,
            || execution.call(prompt),
        )
        .await;

        // Quality-gate retry loop: if agent succeeded but quality gates fail,
        // feed errors back and let the agent try to fix them.
        let (final_result, quality_passed, failure_reason) = execution
            .finish_quality(
                call_result,
                self.quality_checks_enabled,
                &self.quality_checks,
                self.max_quality_retries,
            )
            .await;

        (final_result, quality_passed, failure_reason)
    }
}
