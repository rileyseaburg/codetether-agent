//! Model discovery using the application's Vault-backed provider registry.
use crate::{Application, Provider, ProviderRegistry};
use codetether_models_cli::ModelsArgs;
use std::sync::Arc;

/// Initialize providers and execute model capability discovery.
/// # Arguments
/// * `args` - Provider filter and output format for the models command.
/// # Returns
/// Unit after the shared command finishes presenting its results.
/// # Errors
/// Returns provider initialization or command serialization errors.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>() -> anyhow::Result<()> {
/// codetether_application_providers::cli::models::execute::<H>(
///     codetether_models_cli::ModelsArgs { provider: None, json: true },
/// ).await
/// # }
/// ```
pub async fn execute<H: Application>(args: ModelsArgs) -> anyhow::Result<()> {
    let registry = ProviderRegistry::<H>::from_vault().await?;
    codetether_models_cli::execute(args, &registry).await
}

impl<H: Application> codetether_models_cli::Registry for ProviderRegistry<H> {
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
}
