//! Construct providers and tool registries for the shared stdio serving flow.
use super::ApplicationHost;
use crate::serve::ProviderHost;
use codetether_application_providers::{Application as ProviderApplication, ProviderRegistry};
use codetether_application_registry::{Application as ToolApplication, ToolRegistry};
use codetether_provider_core::Provider;
use std::sync::Arc;
impl<P: ProviderApplication, T: ToolApplication, S> ProviderHost
    for ApplicationHost<ProviderRegistry<P>, ToolRegistry<T>, S>
{
    type Providers = ProviderRegistry<P>;
    type Tools = ToolRegistry<T>;
    async fn providers() -> anyhow::Result<ProviderRegistry<P>> {
        ProviderRegistry::from_vault().await
    }
    fn get(providers: &ProviderRegistry<P>, name: &str) -> Option<Arc<dyn Provider>> {
        providers.get(name)
    }
    fn provider_tools(provider: Arc<dyn Provider>, model: String) -> Arc<ToolRegistry<T>> {
        Arc::new(ToolRegistry::with_provider(provider, model))
    }
    fn default_tools() -> Arc<ToolRegistry<T>> {
        Arc::new(ToolRegistry::with_defaults())
    }
}
