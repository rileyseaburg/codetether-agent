//! Preserve Ralph quality-gate names and their execution order.
use super::{Application, BenchmarkRunner};
use codetether_benchmark::QualityCheckResult;
use codetether_ralph_core::QualityChecks;
use std::path::Path;
impl<H: Application> BenchmarkRunner<H> {
    pub(super) fn run_quality_checks(
        working_dir: &Path,
        checks: &QualityChecks,
    ) -> Vec<QualityCheckResult> {
        codetether_benchmark::quality::run_quality_checks(
            working_dir,
            &[
                ("typecheck", &checks.typecheck),
                ("lint", &checks.lint),
                ("test", &checks.test),
                ("build", &checks.build),
            ],
        )
    }
}
