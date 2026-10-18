//! Aggregate a provider catalog response or failure.
use crate::{ModelRefreshSummary, load::MODEL_LIST_TIMEOUT};
use codetether_provider_core::ModelInfo;
pub(crate) type FetchResult = Result<anyhow::Result<Vec<ModelInfo>>, tokio::time::error::Elapsed>;
pub(crate) fn apply(
    models: &mut Vec<String>,
    summary: &mut ModelRefreshSummary,
    provider_name: &str,
    result: FetchResult,
) {
    match result {
        Ok(Ok(provider_models)) => {
            super::extend_models::extend(models, summary, provider_name, provider_models);
        }
        Ok(Err(err)) => {
            summary.failed_providers += 1;
            tracing::warn!(
                provider = %provider_name,
                error = %err,
                "failed to load models for TUI picker"
            );
        }
        Err(_) => {
            summary.timed_out_providers += 1;
            tracing::warn!(
                provider = %provider_name,
                timeout_ms = MODEL_LIST_TIMEOUT.as_millis(),
                "timed out loading models for TUI picker"
            );
        }
    }
}
