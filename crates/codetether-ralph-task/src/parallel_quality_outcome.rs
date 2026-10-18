//! Turn an initial model result into a verified or failed task outcome.
use super::parallel_execution_context::ParallelExecutionContext;
use super::*;
impl<H: Host> ParallelExecutionContext<'_, H> {
    pub(crate) async fn finish_quality(
        &self,
        call_result: anyhow::Result<String>,
        quality_checks_enabled: bool,
        quality_checks: &QualityChecks,
        max_quality_retries: usize,
    ) -> (anyhow::Result<String>, bool, Option<String>) {
        codetether_ralph_core::quality_repair::finish(
            self,
            call_result,
            quality_checks_enabled,
            quality_checks,
            max_quality_retries,
        )
        .await
    }
}
