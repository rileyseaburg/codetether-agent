//! Suite discovery and sequential model scheduling.
use super::BenchmarkBackend;
use crate::{BenchmarkConfig, BenchmarkSuiteResult};
use std::time::Instant;
use tracing::info;
/// Run all configured models, score results, write output and optionally submit.
///
/// # Arguments
/// * `config` - Discovery, limits and output settings.
/// * `backend` - Host provider and isolated-PRD integration.
/// * `app_version` - Application version recorded in the report.
/// # Returns
/// Completed suite report after writing its JSON output.
/// # Errors
/// Returns discovery, model-loading, serialization or output-writing errors.
/// # Examples
/// ```no_run
/// # async fn example<B: codetether_benchmark::suite::BenchmarkBackend>(backend: &B) -> anyhow::Result<()> {
/// let config = codetether_benchmark::BenchmarkConfig::default();
/// let report = codetether_benchmark::suite::run_suite(&config, backend, "1.0").await?;
/// assert_eq!(report.agent_version, "1.0");
/// # Ok(()) }
/// ```
pub async fn run_suite<B: BenchmarkBackend + ?Sized>(
    config: &BenchmarkConfig,
    backend: &B,
    app_version: &str,
) -> anyhow::Result<BenchmarkSuiteResult> {
    let start = Instant::now();
    let prds = crate::discover_prds(config)?;

    if prds.is_empty() {
        anyhow::bail!("No benchmark PRDs found in {}", config.prd_dir);
    }

    info!("Discovered {} benchmark PRDs across tiers", prds.len());

    let mut model_results = Vec::new();

    for model_str in &config.models {
        model_results.push(super::model::run_model(config, backend, model_str, &prds).await?);
    }
    super::finish::finish(config, model_results, start, app_version).await
}
