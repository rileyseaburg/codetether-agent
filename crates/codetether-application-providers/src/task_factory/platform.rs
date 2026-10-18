//! Reuse the same cached provider registry for task-key fallback.
use super::TaskFactory;
use crate::{Application, ProviderRegistry};
impl<H: Application> codetether_provider_registry::task::Host for TaskFactory<H> {
    async fn platform_registry() -> anyhow::Result<codetether_provider_registry::ProviderRegistry> {
        Ok(ProviderRegistry::<H>::shared_from_vault()
            .await?
            .as_shared()
            .clone())
    }
}
