//! Network-gated provider routing for semantic search planning.

use super::super::{model::resolve_router_model, parse, request};
use crate::provider::{Provider, ProviderRegistry};
use anyhow::{Context, Result};
use std::sync::Arc;

pub(super) async fn resolve(
    registry: &ProviderRegistry,
    router_model: &str,
    query: &str,
    top_n: usize,
) -> Result<(Arc<dyn Provider>, String, parse::RouterPlan)> {
    let (provider, model_id) = resolve_router_model(registry, router_model)?;
    let request = request::build_router_request(&model_id, query, top_n.max(1));
    let response = provider
        .complete(request)
        .await
        .context("router model call failed")?;
    let raw = super::collect_text(&response);
    let plan = parse::parse_router_response(&raw)?;
    Ok((provider, model_id, plan))
}
