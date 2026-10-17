//! Sequential PRD scheduling and per-model cost ceiling.
use super::BenchmarkBackend;
use crate::{BenchmarkConfig, PrdBenchmarkResult, pricing::ModelPricing};
use std::path::PathBuf;
use tracing::{info, warn};
pub(super) async fn run_prds<B: BenchmarkBackend + ?Sized>(
    config: &BenchmarkConfig,
    backend: &B,
    model: &B::Model,
    model_str: &str,
    model_name: &str,
    pricing: &Option<ModelPricing>,
    prds: &[(PathBuf, u8)],
) -> Vec<PrdBenchmarkResult> {
    let mut prd_results = Vec::new();
    let mut total_cost = 0.0_f64;

    for (prd_path, tier) in prds {
        // Check cost ceiling before starting
        if let Some(ceiling) = config.cost_ceiling_usd
            && total_cost >= ceiling
        {
            warn!(
                "Cost ceiling ${:.2} reached — skipping remaining PRDs for model {}",
                ceiling, model_str
            );
            break;
        }

        let prd_id = prd_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        info!("Running benchmark PRD: {} (tier {})", prd_id, tier);

        let outcome = backend
            .run_prd(model, prd_path, *tier, model_name, pricing)
            .await;
        super::record::record(outcome, prd_id, *tier, &mut total_cost, &mut prd_results);
    }

    prd_results
}
