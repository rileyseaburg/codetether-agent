//! openai provider registry integration.
use crate::{Application, Provider, ProviderRegistry};
use std::sync::Arc;
impl<H: Application> codetether_openai_serving::http::Registry for ProviderRegistry<H> {
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
}

/// OpenAI HTTP provider host using application `H`'s factory and shared cache.
///
/// This marker is selected as a type; each request reuses the same cached
/// registry through the application's established Vault/configuration loader.
///
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>() -> anyhow::Result<()> {
/// use codetether_openai_serving::http::Host;
/// let _registry = codetether_application_providers::OpenAiHttpHost::<H>::load_registry().await?;
/// # Ok(()) }
/// ```
pub struct OpenAiHttpHost<H>(std::marker::PhantomData<fn() -> H>);

impl<H: Application> codetether_openai_serving::http::Host for OpenAiHttpHost<H> {
    type Registry = ProviderRegistry<H>;
    type Error = anyhow::Error;
    async fn load_registry() -> Result<Arc<ProviderRegistry<H>>, anyhow::Error> {
        ProviderRegistry::shared_from_vault().await
    }
}
