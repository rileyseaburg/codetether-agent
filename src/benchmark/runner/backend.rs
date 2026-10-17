//! Provider and Ralph execution adapter for the shared suite scheduler.
use super::BenchmarkRunner;
use crate::provider::ProviderRegistry;
use codetether_benchmark::{PrdBenchmarkResult, pricing::ModelPricing};
use std::path::Path;
pub(super) struct RunnerBackend<'a>(pub(super) &'a BenchmarkRunner);
pub(super) struct PreparedModel {
    provider: std::sync::Arc<dyn crate::provider::Provider>,
    _registry: ProviderRegistry,
}
#[async_trait::async_trait]
impl codetether_benchmark::suite::BenchmarkBackend for RunnerBackend<'_> {
    type Model = PreparedModel;
    async fn prepare_model(&self, provider_name: &str) -> anyhow::Result<Self::Model> {
        let registry = ProviderRegistry::from_vault().await?;
        let provider = registry
            .get(provider_name)
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
