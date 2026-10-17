//! Host operations required by the quality-repair state machine.
use super::*;
/// Supplies application checks, verification, repair and reporting.
///
/// # Implementors
/// Implement this at the runtime boundary. Each callback is invoked in order;
/// check and verification failures may be followed by a repair attempt.
#[async_trait::async_trait]
pub trait RepairHost: Sync {
    /// Run configured checks and return success plus diagnostic output.
    async fn check(&self, checks: &QualityChecks) -> (bool, String);
    /// Verify a successful check attempt and update its evidence.
    async fn verify(&self, state: &mut QualityProgress, error: &mut String, attempt: usize)
    -> bool;
    /// Run repair, updating output; return false to stop retrying.
    async fn repair(
        &self,
        state: &mut QualityProgress,
        error: &str,
        attempt: usize,
        maximum: usize,
    ) -> bool;
    /// Record exhaustion of the final allowed check attempt.
    fn exhausted(&self, state: &mut QualityProgress, error: &str, maximum: usize);
    /// Verify the story when configured quality commands are disabled.
    async fn verify_without_quality(&self, state: &mut QualityProgress);
}
