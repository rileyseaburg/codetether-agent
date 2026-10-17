//! Host boundary for model loading and PRD execution.
use crate::{PrdBenchmarkResult, pricing::ModelPricing};
use std::path::Path;
/// Supplies application integration to the sequential suite runner.
///
/// # Implementors
/// Hosts load their provider state in prepare_model and execute an isolated
/// PRD in run_prd. The model state stays alive for all PRDs of that model.
#[async_trait::async_trait]
pub trait BenchmarkBackend: Sync {
    /// Loaded model state retained for one model's benchmarks.
    type Model: Send + Sync;
    /// Load provider state before pricing lookup or PRD execution.
    ///
    /// # Arguments
    /// * `provider_name` - Provider parsed from the configured model string.
    /// # Returns
    /// State owned by this model run.
    /// # Errors
    /// Returns the host's provider-loading error, aborting the suite.
    async fn prepare_model(&self, provider_name: &str) -> anyhow::Result<Self::Model>;
    /// Execute one isolated PRD; a failure is recorded and the suite continues.
    ///
    /// # Arguments
    /// * `model` - Previously prepared provider state.
    /// * `prd_path` - Source PRD file.
    /// * `tier` - Discovered benchmark tier.
    /// * `model_name` - Model name without its provider prefix.
    /// * `pricing` - Optional token prices for cost reporting.
    /// # Returns
    /// Per-PRD measurements.
    /// # Errors
    /// Returns the host execution error to record as a failed PRD.
    async fn run_prd(
        &self,
        model: &Self::Model,
        prd_path: &Path,
        tier: u8,
        model_name: &str,
        pricing: &Option<ModelPricing>,
    ) -> anyhow::Result<PrdBenchmarkResult>;
}
