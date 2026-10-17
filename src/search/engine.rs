//! Application model resolution and concrete backend adapter for search.
use super::model::resolve_router_model;
use super::result::RouterResult;
use crate::provider::ProviderRegistry;
use crate::tool::ToolResult;
use anyhow::Result;
use codetether_search::types::BackendChoice;
use std::sync::Arc;
struct HostBackends;
#[async_trait::async_trait]
impl codetether_search::BackendExecutor for HostBackends {
    async fn run_choice(&self, choice: &BackendChoice, query: &str) -> Result<ToolResult> {
        super::dispatch::run_choice(choice, query).await
    }
}

/// Execute the full search pipeline.
///
/// Picks up to `top_n` backends from the router's plan (1 when the caller
/// wants a single best hit; >1 for fan-out).
pub async fn run_router_search(
    registry: Arc<ProviderRegistry>,
    router_model: &str,
    query: &str,
    top_n: usize,
) -> Result<RouterResult> {
    let (provider, model_id) = resolve_router_model(&registry, router_model)?;
    codetether_search::run_router_search(provider, model_id, query, top_n, &HostBackends).await
}
