//! Application provider and embedding policy for the shared search CLI.
use crate::cli::SearchArgs;
use crate::provider::ProviderRegistry;
use crate::tool::memory::ApplicationAutoEmbedder;
use std::sync::Arc;

/// Execute search using providers initialized from the application's configuration.
/// # Errors
/// Returns provider initialization or search errors.
pub async fn execute(args: SearchArgs) -> anyhow::Result<()> {
    let providers = Arc::new(ProviderRegistry::from_vault().await?);
    codetether_search_cli::execute::<_, ApplicationAutoEmbedder>(args, providers).await
}
