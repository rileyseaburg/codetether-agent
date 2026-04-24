//! `search` router tool — LLM picks a backend, runs it, returns JSON.
//!
//! This is a thin `Tool` wrapper over [`crate::search::run_router_search`]
//! so agents can call the same pipeline the CLI uses.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

use super::{Tool, ToolResult};
use crate::provider::ProviderRegistry;
use crate::search::{model::DEFAULT_ROUTER_MODEL, run_router_search};

/// Search-router tool. Requires a [`ProviderRegistry`] so the LLM router
/// can pick the backend.
pub struct SearchTool {
    registry: Arc<ProviderRegistry>,
}

impl SearchTool {
    pub fn new(registry: Arc<ProviderRegistry>) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl Tool for SearchTool {
    fn id(&self) -> &str {
        "search"
    }
    fn name(&self) -> &str {
        "Search Router"
    }
    fn description(&self) -> &str {
        "search(query: string, top_n?: int, router_model?: string) — LLM-routed search. Picks grep/glob/websearch/webfetch/memory/rlm based on the query and returns normalized JSON."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {"type": "string", "description": "Natural-language search query"},
                "top_n": {"type": "integer", "description": "Max backends to run (default 1)"},
                "router_model": {"type": "string", "description": "Override router model (default zai/glm-5.1)"}
            },
            "required": ["query"]
        })
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
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
        match run_router_search(Arc::clone(&self.registry), router_model, query, top_n).await {
            Ok(result) => {
                let payload = serde_json::to_string_pretty(&result)?;
                Ok(ToolResult::success(payload)
                    .with_metadata("backends", json!(result.runs.len()))
                    .with_metadata("router_model", json!(result.router_model)))
            }
            Err(err) => Ok(ToolResult::error(format!("search router failed: {err}"))),
        }
    }
}
