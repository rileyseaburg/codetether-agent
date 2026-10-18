//! Task-specific credentials with the existing cached platform fallback.
use super::*;
impl<H: Application> ProviderRegistry<H> {
    /// Prefer usable task credentials, otherwise load the cached platform registry.
    /// # Errors
    /// Propagates the existing task/platform initialization error.
    /// # Arguments
    /// `keys` is the optional task credential claim.
    /// # Returns
    /// A task-specific registry or the cached platform fallback.
    /// # Examples
    /// ```no_run
    /// # async fn example<H: codetether_application_providers::Application>() -> anyhow::Result<()> {
    /// let _registry = codetether_application_providers::ProviderRegistry::<H>::for_task_keys(None).await?;
    /// # Ok(()) }
    /// ```
    pub async fn for_task_keys(keys: Option<&serde_json::Value>) -> Result<Self> {
        codetether_provider_registry::task::load_task_provider_registry::<
            task_factory::TaskFactory<H>,
        >(keys)
        .await
        .map(Self::from)
    }
}
