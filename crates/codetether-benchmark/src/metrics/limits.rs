//! Normalization limits for benchmark model ranking.
use super::*;
pub(super) fn limits(model_results: &[ModelBenchmarkResult]) -> (f64, f64, f64) {
    // Find max values for normalization
    let max_pass_rate = model_results
        .iter()
        .map(|r| r.aggregate.overall_pass_rate)
        .fold(0.0_f64, f64::max);
    let max_speed = model_results
        .iter()
        .map(|r| r.aggregate.stories_per_hour)
        .fold(0.0_f64, f64::max);
    let min_cost = model_results
        .iter()
        .filter(|r| r.aggregate.avg_cost_per_story > 0.0)
        .map(|r| r.aggregate.avg_cost_per_story)
        .fold(f64::INFINITY, f64::min);

    (max_pass_rate, max_speed, min_cost)
}
