//! Bind shared search orchestration to the application's provider and memory policy.
use super::result::RouterResult;
use crate::provider::ProviderRegistry;
use crate::tool::memory::ApplicationAutoEmbedder;
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
/// # async fn example(registry: std::sync::Arc<codetether_agent::provider::ProviderRegistry>) -> anyhow::Result<()> {
/// let result = codetether_agent::search::run_router_search(registry, "zai/glm-5.3", "find main", 1).await?;
/// assert_eq!(result.query, "find main");
/// # Ok(()) }
/// ```
pub async fn run_router_search(
    registry: Arc<ProviderRegistry>,
    router_model: &str,
    query: &str,
    top_n: usize,
) -> anyhow::Result<RouterResult> {
    codetether_search_router::run_router_search::<_, ApplicationAutoEmbedder>(
        registry,
        router_model,
        query,
        top_n,
    )
    .await
}
