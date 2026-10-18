//! Access the native process-wide provider cache for task registry fallback.
use crate::provider::{ProviderRegistry, init_dispatch::host::Native};
impl codetether_provider_registry::task::Host for Native {
    async fn platform_registry() -> anyhow::Result<codetether_provider_registry::ProviderRegistry> {
        Ok(ProviderRegistry::shared_from_vault()
            .await?
            .as_shared()
            .clone())
    }
}
