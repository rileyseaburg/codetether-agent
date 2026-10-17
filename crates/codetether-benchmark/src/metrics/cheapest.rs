//! Select the cheapest model using the original tie-breaking behavior.
use super::*;
pub(super) fn cheapest(model_results: &[ModelBenchmarkResult]) -> String {
    model_results
        .iter()
        .filter(|r| r.aggregate.avg_cost_per_story > 0.0)
        .min_by(|a, b| {
            a.aggregate
                .avg_cost_per_story
                .partial_cmp(&b.aggregate.avg_cost_per_story)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|r| r.model.clone())
        .unwrap_or_default()
}
