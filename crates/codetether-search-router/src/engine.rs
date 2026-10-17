//! Combine registry model resolution with the shared planning engine.
use crate::{Registry, model::resolve_router_model};
use anyhow::Result;
use codetether_memory_tool::AutoEmbedder;
use codetether_search::{BackendChoice, BackendExecutor, RouterResult};
use codetether_tool_core::ToolResult;
use std::{marker::PhantomData, sync::Arc};
struct Backends<B>(PhantomData<B>);
#[async_trait::async_trait]
impl<B: AutoEmbedder> BackendExecutor for Backends<B> {
    async fn run_choice(&self, choice: &BackendChoice, query: &str) -> Result<ToolResult> {
        crate::dispatch::run_choice::<B>(choice, query).await
    }
}
/// Resolve the router model, plan a search and execute selected backends.
///
/// # Arguments
/// * `registry` - Shared configured provider registry.
/// * `router_model` - Requested provider/model identity.
/// * `query` - Search question.
/// * `top_n` - Maximum selected backends, clamped to one by the planning engine.
/// # Returns
/// Ordered backend results with the resolved router identity.
/// # Errors
/// Returns resolution, planning or backend errors.
/// # Examples
/// ```rust,no_run
/// # async fn example<R:codetether_search_router::Registry>(registry:std::sync::Arc<R>)->anyhow::Result<()> {
/// let result=codetether_search_router::run_router_search::<R,codetether_memory_tool::NoAutoEmbedder>(registry,"zai/glm-5.3","find main",1).await?;
/// assert_eq!(result.query,"find main");
/// # Ok(()) }
/// ```
pub async fn run_router_search<R: Registry, B: AutoEmbedder>(
    registry: Arc<R>,
    router_model: &str,
    query: &str,
    top_n: usize,
) -> Result<RouterResult> {
    let (provider, model_id) = resolve_router_model(registry.as_ref(), router_model)?;
    codetether_search::run_router_search(
        provider,
        model_id,
        query,
        top_n,
        &Backends::<B>(PhantomData),
    )
    .await
}
