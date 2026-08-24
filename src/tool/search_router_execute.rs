//! Approval-bound search-router execution.

use crate::provider::ProviderRegistry;
use crate::search::{model::DEFAULT_ROUTER_MODEL, run_router_search};
use crate::tool::{ToolResult, network_access};
use anyhow::Result;
use serde_json::{Value, json};
use std::sync::Arc;

pub(super) async fn run(registry: &Arc<ProviderRegistry>, args: Value) -> Result<ToolResult> {
    let query = match args["query"].as_str() {
        Some(query) if !query.is_empty() => query,
        _ => {
            return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "search",
                "query is required",
                Some(vec!["query"]),
                Some(json!({"query": "where is fn main"})),
            ));
        }
    };
    network_access::guard!("search", &args);
    let top_n = args["top_n"].as_u64().unwrap_or(1).max(1) as usize;
    let model = args["router_model"]
        .as_str()
        .unwrap_or(DEFAULT_ROUTER_MODEL);
    match run_router_search(Arc::clone(registry), model, query, top_n).await {
        Ok(result) => {
            let payload = serde_json::to_string_pretty(&result)?;
            Ok(ToolResult::success(payload)
                .with_metadata("backends", json!(result.runs.len()))
                .with_metadata("router_model", json!(result.router_model)))
        }
        Err(error) => Ok(ToolResult::error(format!("search router failed: {error}"))),
    }
}
