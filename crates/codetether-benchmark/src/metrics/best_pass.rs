//! Select the best_pass model using the original tie-breaking behavior.
use super::*;
pub(super) fn best_pass(model_results: &[ModelBenchmarkResult]) -> String {
    model_results
        .iter()
        .max_by(|a, b| {
            a.aggregate
                .overall_pass_rate
                .partial_cmp(&b.aggregate.overall_pass_rate)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|r| r.model.clone())
        .unwrap_or_default()
}
