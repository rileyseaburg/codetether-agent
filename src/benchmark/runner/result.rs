//! Combine Ralph outcomes, measurements and final quality-gate results.
use super::{BenchmarkRunner, measurements::Measurements};
use crate::ralph::{QualityChecks, RalphState};
use codetether_benchmark::PrdBenchmarkResult;
use std::path::Path;
impl BenchmarkRunner {
    pub(super) fn finish_prd(
        state: &RalphState,
        prd_id: String,
        tier: u8,
        prd_feature: String,
        measurements: Measurements,
        workspace_dir: &Path,
        prd_quality_checks: &QualityChecks,
    ) -> PrdBenchmarkResult {
        let Measurements {
            duration,
            tokens_used,
            cost_usd,
        } = measurements;
        let per_story = super::stories::story_results(state);
        let stories_passed = state.prd.passed_count();
        let stories_total = state.prd.user_stories.len();

        // Run final quality checks to capture their state for reporting
        let quality_checks = Self::run_quality_checks(workspace_dir, prd_quality_checks);

        PrdBenchmarkResult {
            prd_id,
            prd_tier: tier,
            prd_feature,
            stories_total,
            stories_passed,
            pass_rate: if stories_total > 0 {
                stories_passed as f64 / stories_total as f64
            } else {
                0.0
            },
            duration_seconds: duration.as_secs_f64(),
            tokens_used,
            cost_usd,
            quality_checks,
            per_story,
        }
    }
}
