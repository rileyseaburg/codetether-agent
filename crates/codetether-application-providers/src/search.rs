//! Bind shared search orchestration to the application's provider and memory policy.
use crate::embedding::AutoEmbedder;
use crate::{Application, ProviderRegistry};
use codetether_search::RouterResult;
use std::sync::Arc;
/// Execute the shared router with the application's automatic embedding selection.
///
/// # Arguments
/// * `registry` - Shared configured provider registry.
/// * `router_model` - Requested provider/model identity.
/// * `query` - Search question.
/// * `top_n` - Maximum number of selected backends.
/// # Returns
/// Ordered backend results and the resolved router identity.
/// # Errors
/// Returns provider resolution, planning or backend errors.
/// # Examples
/// ```rust,no_run
/// # async fn example<H: codetether_application_providers::Application>(registry: std::sync::Arc<codetether_application_providers::ProviderRegistry<H>>) -> anyhow::Result<()> {
/// let result = codetether_application_providers::search::run_router_search(registry, "zai/glm-5.3", "find main", 1).await?;
/// assert_eq!(result.query, "find main");
/// # Ok(()) }
/// ```
pub async fn run_router_search<H: Application>(
    registry: Arc<ProviderRegistry<H>>,
    router_model: &str,
    query: &str,
    top_n: usize,
) -> anyhow::Result<RouterResult> {
    codetether_search_router::run_router_search::<_, AutoEmbedder<ProviderRegistry<H>>>(
        registry,
        router_model,
        query,
        top_n,
    )
    .await
}
