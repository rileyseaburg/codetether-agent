//! Assemble cross-model winners and normalized rankings.
use super::*;

/// Compute summary rankings across all models
///
/// # Arguments
/// * `model_results` - Per-model aggregate measurements.
/// # Returns
/// Winners and normalized weighted rankings.
/// # Examples
/// ```
/// assert!(codetether_benchmark::metrics::compute_summary(&[]).rankings.is_empty());
/// ```
pub fn compute_summary(model_results: &[ModelBenchmarkResult]) -> BenchmarkSummary {
    if model_results.is_empty() {
        return BenchmarkSummary {
            best_pass_rate_model: String::new(),
            fastest_model: String::new(),
            cheapest_model: String::new(),
            best_overall_model: String::new(),
            rankings: Vec::new(),
        };
    }

    let (max_pass_rate, max_speed, min_cost) = limits(model_results);
    let rankings = rank_models(model_results, max_pass_rate, max_speed, min_cost);
    let best_pass = best_pass(model_results);
    let fastest = fastest(model_results);
    let cheapest = cheapest(model_results);
    let best_overall = rankings
        .first()
        .map(|r| r.model.clone())
        .unwrap_or_default();

    BenchmarkSummary {
        best_pass_rate_model: best_pass,
        fastest_model: fastest,
        cheapest_model: cheapest,
        best_overall_model: best_overall,
        rankings,
    }
}
