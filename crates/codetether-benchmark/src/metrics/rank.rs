//! Score and order model measurements with the existing weights.
use super::*;
pub(super) fn rank_models(
    model_results: &[ModelBenchmarkResult],
    max_pass_rate: f64,
    max_speed: f64,
    min_cost: f64,
) -> Vec<ModelRanking> {
    let mut rankings: Vec<ModelRanking> = model_results
        .iter()
        .map(|r| {
            let pass_rate_score = if max_pass_rate > 0.0 {
                (r.aggregate.overall_pass_rate / max_pass_rate) * 100.0
            } else {
                0.0
            };

            let speed_score = if max_speed > 0.0 {
                (r.aggregate.stories_per_hour / max_speed) * 100.0
            } else {
                0.0
            };

            let cost_score = if r.aggregate.avg_cost_per_story > 0.0 && min_cost.is_finite() {
                (min_cost / r.aggregate.avg_cost_per_story) * 100.0
            } else {
                0.0
            };

            // Weighted: 50% accuracy, 25% speed, 25% cost
            let overall_score = pass_rate_score * 0.50 + speed_score * 0.25 + cost_score * 0.25;

            ModelRanking {
                model: r.model.clone(),
                pass_rate_score,
                speed_score,
                cost_score,
                overall_score,
            }
        })
        .collect();

    rankings.sort_by(|a, b| {
        b.overall_score
            .partial_cmp(&a.overall_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    rankings
}
