//! Record successful measurements or the original zero-valued failure result.
use crate::PrdBenchmarkResult;
use tracing::{error, info};
pub(super) fn record(
    outcome: anyhow::Result<PrdBenchmarkResult>,
    prd_id: String,
    tier: u8,
    total_cost: &mut f64,
    prd_results: &mut Vec<PrdBenchmarkResult>,
) {
    match outcome {
        Ok(result) => {
            *total_cost += result.cost_usd;
            info!(
                "PRD {} complete: {}/{} stories passed ({:.0}%) — ${:.4}",
                prd_id,
                result.stories_passed,
                result.stories_total,
                result.pass_rate * 100.0,
                result.cost_usd,
            );
            prd_results.push(result);
        }
        Err(e) => {
            error!("Failed to run benchmark PRD {}: {}", prd_id, e);
            prd_results.push(PrdBenchmarkResult {
                prd_id,
                prd_tier: tier,
                prd_feature: "FAILED".to_string(),
                stories_total: 0,
                stories_passed: 0,
                pass_rate: 0.0,
                duration_seconds: 0.0,
                tokens_used: 0,
                cost_usd: 0.0,
                quality_checks: Vec::new(),
                per_story: Vec::new(),
            });
        }
    }
}
