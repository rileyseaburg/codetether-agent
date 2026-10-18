//! Search command initialization with application provider and embedding policy.
use crate::{Application, ProviderRegistry, embedding::AutoEmbedder};
use codetether_search_cli::SearchArgs;
use std::sync::Arc;

/// Load application providers and execute search with automatic embeddings.
/// # Arguments
/// * `args` - Query, router selection, result count and output format.
/// # Returns
/// Unit after the search command finishes presenting its results.
/// # Errors
/// Returns provider initialization, search or result serialization errors.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>() -> anyhow::Result<()> {
/// let args = codetether_search_cli::SearchArgs {
///     query: "find main".into(), router_model: None, top_n: 1, json: true,
/// };
/// codetether_application_providers::cli::search::execute::<H>(args).await
/// # }
/// ```
pub async fn execute<H: Application>(args: SearchArgs) -> anyhow::Result<()> {
    let providers = Arc::new(ProviderRegistry::<H>::from_vault().await?);
    codetether_search_cli::execute::<_, AutoEmbedder<ProviderRegistry<H>>>(args, providers).await
}
