//! Ask a resolved model for a plan, execute its choices, and collect results.

use anyhow::{Context, Result};
use std::sync::Arc;

use codetether_provider_core::Provider;

use super::backend::BackendExecutor;
use super::collect::collect_text;

use super::parse::parse_router_response;
use super::request::build_router_request;
use super::result::{BackendRun, RouterResult};

/// Execute the full search pipeline.
///
/// Picks up to `top_n` backends from the router's plan (1 when the caller
/// wants a single best hit; >1 for fan-out).
///
/// # Arguments
/// * `provider` - Resolved provider used for the planning request.
/// * `model_id` - Provider-local model identifier.
/// * `query` - Original natural-language search query.
/// * `top_n` - Maximum backend count, clamped to at least one.
/// * `executor` - Host adapter that executes each selected backend.
/// # Returns
/// Results in the model's selected order, with the resolved model identity.
/// # Errors
/// Returns provider, plan parsing, or backend execution errors.
/// # Examples
/// ```no_run
/// # async fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>,
/// # executor: &dyn codetether_search::BackendExecutor) -> anyhow::Result<()> {
/// let result = codetether_search::run_router_search(
///     provider, "model".into(), "find main", 1, executor,
/// ).await?;
/// assert_eq!(result.query, "find main");
/// # Ok(()) }
/// ```
pub async fn run_router_search(
    provider: Arc<dyn Provider>,
    model_id: String,
    query: &str,
    top_n: usize,
    executor: &dyn BackendExecutor,
) -> Result<RouterResult> {
    let request = build_router_request(&model_id, query, top_n.max(1));
    let response = provider
        .complete(request)
        .await
        .context("router model call failed")?;
    let raw = collect_text(&response);
    let plan = parse_router_response(&raw)?;
    let mut runs = Vec::with_capacity(plan.choices.len().min(top_n.max(1)));
    for choice in plan.choices.into_iter().take(top_n.max(1)) {
        let tool_result = executor.run_choice(&choice, query).await?;
        runs.push(BackendRun {
            backend: choice.backend,
            success: tool_result.success,
            output: tool_result.output,
            metadata: serde_json::to_value(tool_result.metadata).unwrap_or_default(),
        });
    }
    Ok(RouterResult {
        query: query.to_string(),
        router_model: format!("{}/{}", provider.name(), model_id),
        runs,
    })
}
