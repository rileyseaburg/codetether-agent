//! Native provider initialization for the shared RLM command.
use crate::{cli, provider};
struct NativeProviderSource;
#[async_trait::async_trait]
impl codetether_rlm_cli::ProviderSource for NativeProviderSource {
    async fn select(
        model_arg: Option<&str>,
    ) -> anyhow::Result<codetether_rlm_selection::SelectedProvider> {
        use provider::ProviderRegistry;
        let registry = ProviderRegistry::from_vault().await.map_err(|e| {
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
pub(super) async fn run(args: cli::RlmArgs) -> anyhow::Result<()> {
    codetether_rlm_cli::execute::<NativeProviderSource>(args).await
}
