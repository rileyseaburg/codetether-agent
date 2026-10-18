//! Application shell, verification and repair callbacks for shared retry sequencing.
use super::parallel_execution_context::ParallelExecutionContext;
use super::*;
use codetether_ralph_core::quality_repair::RepairHost;
#[async_trait::async_trait]
impl<H: Host> RepairHost for ParallelExecutionContext<'_, H> {
    async fn check(&self, checks: &QualityChecks) -> (bool, String) {
        H::check(self.working_dir, checks).await
    }
    async fn verify(
        &self,
        state: &mut QualityProgress,
        error: &mut String,
        attempt: usize,
    ) -> bool {
        self.verify_quality_attempt(state, error, attempt).await
    }
    async fn repair(
        &self,
        state: &mut QualityProgress,
        error: &str,
        attempt: usize,
        maximum: usize,
    ) -> bool {
        self.run_quality_repair(state, error, attempt, maximum)
            .await
    }
    fn exhausted(&self, state: &mut QualityProgress, error: &str, maximum: usize) {
        self.quality_exhausted(state, error, maximum);
    }
    async fn verify_without_quality(&self, state: &mut QualityProgress) {
        ParallelExecutionContext::verify_without_quality(self, state).await
    }
}
