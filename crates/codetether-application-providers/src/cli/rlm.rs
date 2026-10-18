//! RLM provider loading and selection through the application registry.
use crate::{Application, ProviderRegistry};

#[async_trait::async_trait]
impl<H: Application> codetether_rlm_cli::ProviderSource for ProviderRegistry<H> {
    async fn select(
        model_arg: Option<&str>,
    ) -> anyhow::Result<codetether_rlm_selection::SelectedProvider> {
        let registry = ProviderRegistry::<H>::from_vault().await.map_err(|e| {
            anyhow::anyhow!(
                "Failed to load providers for Oracle-First RLM: {}. \
             Configure local_cuda or openrouter.",
                e
            )
        })?;
        let (provider, model, provider_name) =
            codetether_rlm_selection::resolve(model_arg, &|name| registry.get(name))?;
        Ok((provider, model, provider_name))
    }
}
