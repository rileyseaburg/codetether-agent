//! Provider preparation, pricing and aggregation for one model.
use super::BenchmarkBackend;
use crate::{BenchmarkConfig, ModelBenchmarkResult};
use std::path::PathBuf;
use tracing::info;
pub(super) async fn run_model<B: BenchmarkBackend + ?Sized>(
    config: &BenchmarkConfig,
    backend: &B,
    model_str: &str,
    prds: &[(PathBuf, u8)],
) -> anyhow::Result<ModelBenchmarkResult> {
    let (provider_name, model_name) = crate::parse_model(model_str)?;
    info!("Benchmarking model: {}:{}", provider_name, model_name);

    let model = backend.prepare_model(&provider_name).await?;
    let pricing = super::pricing::fetch_pricing(&provider_name, &model_name).await;
    let prd_results = super::prds::run_prds(
        config,
        backend,
        &model,
        model_str,
        &model_name,
        &pricing,
        prds,
    )
    .await;
    let aggregate = crate::metrics::compute_aggregate(&prd_results);
    Ok(ModelBenchmarkResult {
        model: model_str.to_string(),
        prd_results,
        aggregate,
    })
}
