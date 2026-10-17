//! Preserve Ralph quality-gate names and their execution order.
use super::BenchmarkRunner;
use crate::ralph::QualityChecks;
use codetether_benchmark::QualityCheckResult;
use std::path::Path;
impl BenchmarkRunner {
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
