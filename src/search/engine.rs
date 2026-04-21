//! End-to-end: resolve model → ask router → run chosen backends → collect.

use anyhow::{Context, Result};
use std::sync::Arc;

use crate::provider::ProviderRegistry;

use super::dispatch::run_choice;
use super::model::resolve_router_model;
use super::parse::parse_router_response;
use super::request::build_router_request;
use super::result::{BackendRun, RouterResult};

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
    let request = build_router_request(&model_id, query, top_n.max(1));
    let response = provider
        .complete(request)
        .await
        .context("router model call failed")?;
    let raw = collect_text(&response);
    let plan = parse_router_response(&raw)?;
    let mut runs = Vec::with_capacity(plan.choices.len().min(top_n.max(1)));
    for choice in plan.choices.into_iter().take(top_n.max(1)) {
        let tool_result = run_choice(&choice, query).await?;
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

fn collect_text(response: &crate::provider::CompletionResponse) -> String {
    response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            crate::provider::ContentPart::Text { text }
            | crate::provider::ContentPart::Thinking { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
