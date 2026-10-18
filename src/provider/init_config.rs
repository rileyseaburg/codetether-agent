//! Construct the native registry through shared configuration policy.
use super::registry::ProviderRegistry;
use anyhow::Result;
impl ProviderRegistry {
    /// Initialize providers from application configuration and allowed fallbacks.
    /// # Errors
    /// Returns the first configured provider construction failure.
    pub async fn from_config(config: &crate::config::Config) -> Result<Self> {
        Ok(Self {
            inner: codetether_provider_registry::ProviderRegistry::from_config::<
                super::init_dispatch::host::Native,
            >(config)
            .await?,
        })
    }
}
