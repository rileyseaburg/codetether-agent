//! Parse tool arguments and format normalized search results.
use crate::{Registry, engine::run_router_search, model::DEFAULT_ROUTER_MODEL};
use anyhow::Result;
use codetether_memory_tool::AutoEmbedder;
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
use std::sync::Arc;
pub(super) async fn run<R: Registry, B: AutoEmbedder>(
    registry: &Arc<R>,
    args: Value,
) -> Result<ToolResult> {
    let query = match args["query"].as_str() {
        Some(q) if !q.is_empty() => q,
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
    let top_n = args["top_n"].as_u64().unwrap_or(1).max(1) as usize;
    let router_model = args["router_model"]
        .as_str()
        .unwrap_or(DEFAULT_ROUTER_MODEL);
    match run_router_search::<R, B>(Arc::clone(registry), router_model, query, top_n).await {
        Ok(result) => {
            let payload = serde_json::to_string_pretty(&result)?;
            Ok(ToolResult::success(payload)
                .with_metadata("backends", json!(result.runs.len()))
                .with_metadata("router_model", json!(result.router_model)))
        }
        Err(err) => Ok(ToolResult::error(format!("search router failed: {err}"))),
    }
}
