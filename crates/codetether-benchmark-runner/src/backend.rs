//! Provider and Ralph execution adapter for the shared suite scheduler.
use super::{Application, BenchmarkRunner};
use codetether_benchmark::{PrdBenchmarkResult, pricing::ModelPricing};
use std::path::Path;
pub(super) struct RunnerBackend<'a, H: Application>(pub(super) &'a BenchmarkRunner<H>);
pub(super) struct PreparedModel<H: Application> {
    provider: std::sync::Arc<dyn codetether_provider_core::Provider>,
    _registry: H::Registry,
}
#[async_trait::async_trait]
impl<H: Application> codetether_benchmark::suite::BenchmarkBackend for RunnerBackend<'_, H> {
    type Model = PreparedModel<H>;
    async fn prepare_model(&self, provider_name: &str) -> anyhow::Result<Self::Model> {
        let registry = H::registry().await?;
        let provider = H::provider(&registry, provider_name)
            .ok_or_else(|| anyhow::anyhow!("Provider '{}' not found in Vault", provider_name))?;

        Ok(PreparedModel {
            _registry: registry,
            provider,
        })
    }
    async fn run_prd(
        &self,
        model: &Self::Model,
        prd_path: &Path,
        tier: u8,
        model_name: &str,
        pricing: &Option<ModelPricing>,
    ) -> anyhow::Result<PrdBenchmarkResult> {
        self.0
            .run_single_prd(prd_path, tier, model.provider.clone(), model_name, pricing)
            .await
    }
}
