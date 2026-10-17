//! Select the fastest model using the original tie-breaking behavior.
use super::*;
pub(super) fn fastest(model_results: &[ModelBenchmarkResult]) -> String {
    model_results
        .iter()
        .max_by(|a, b| {
            a.aggregate
                .stories_per_hour
                .partial_cmp(&b.aggregate.stories_per_hour)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|r| r.model.clone())
        .unwrap_or_default()
}
