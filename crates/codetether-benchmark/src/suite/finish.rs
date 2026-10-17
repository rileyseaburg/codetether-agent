//! Score, timestamp, persist and optionally publish a completed suite.
use crate::{BenchmarkConfig, BenchmarkSuiteResult, ModelBenchmarkResult};
use std::{path::Path, time::Instant};
use tracing::info;
pub(super) async fn finish(
    config: &BenchmarkConfig,
    model_results: Vec<ModelBenchmarkResult>,
    start: Instant,
    app_version: &str,
) -> anyhow::Result<BenchmarkSuiteResult> {
    let summary = crate::metrics::compute_summary(&model_results);
    let elapsed = start.elapsed();

    let result = BenchmarkSuiteResult {
        run_date: chrono::Utc::now().to_rfc3339(),
        agent: "codetether".to_string(),
        agent_version: app_version.to_string(),
        model_results,
        summary,
    };

    info!("Benchmark suite complete in {:.1}s", elapsed.as_secs_f64());

    // Write results to file
    let output_path = Path::new(&config.output);
    let json = serde_json::to_string_pretty(&result)?;
    tokio::fs::write(output_path, &json).await?;
    info!("Results written to {}", config.output);

    // Submit to API if configured
    if let (Some(api_url), Some(api_key)) = (&config.submit_api_url, &config.submit_api_key) {
        crate::http::submit_results(&result, api_url, api_key).await;
    }

    Ok(result)
}
