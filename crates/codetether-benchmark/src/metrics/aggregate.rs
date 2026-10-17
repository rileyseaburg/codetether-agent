//! Aggregate measurements without altering benchmark scoring formulas.
use super::*;

/// Compute aggregate metrics from PRD results
///
/// # Arguments
/// * `prd_results` - Completed PRD measurements.
/// # Returns
/// Totals, averages, pass rate, and throughput using the existing formulas.
/// # Examples
/// ```
/// assert_eq!(codetether_benchmark::metrics::compute_aggregate(&[]).prds_attempted, 0);
/// ```
pub fn compute_aggregate(prd_results: &[PrdBenchmarkResult]) -> AggregateMetrics {
    let prds_attempted = prd_results.len();
    let prds_fully_passed = prd_results.iter().filter(|r| r.pass_rate >= 1.0).count();

    let total_stories: usize = prd_results.iter().map(|r| r.stories_total).sum();
    let total_passed: usize = prd_results.iter().map(|r| r.stories_passed).sum();
    let total_tokens: u64 = prd_results.iter().map(|r| r.tokens_used).sum();
    let total_cost: f64 = prd_results.iter().map(|r| r.cost_usd).sum();
    let total_duration: f64 = prd_results.iter().map(|r| r.duration_seconds).sum();

    let overall_pass_rate = if total_stories > 0 {
        total_passed as f64 / total_stories as f64
    } else {
        0.0
    };

    let avg_seconds_per_story = if total_passed > 0 {
        total_duration / total_passed as f64
    } else {
        0.0
    };

    let avg_tokens_per_story = if total_passed > 0 {
        total_tokens as f64 / total_passed as f64
    } else {
        0.0
    };

    let stories_per_hour = if total_duration > 0.0 {
        total_passed as f64 / (total_duration / 3600.0)
    } else {
        0.0
    };

    AggregateMetrics {
        prds_attempted,
        prds_fully_passed,
        overall_pass_rate,
        total_stories,
        total_stories_passed: total_passed,
        avg_seconds_per_story,
        avg_tokens_per_story,
        total_cost_usd: total_cost,
        avg_cost_per_story: if total_passed > 0 {
            total_cost / total_passed as f64
        } else {
            0.0
        },
        total_duration_seconds: total_duration,
        stories_per_hour,
    }
}
