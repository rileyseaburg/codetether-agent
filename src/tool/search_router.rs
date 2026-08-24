//! `search` router tool — LLM picks a backend, runs it, returns JSON.
//!
//! This is a thin `Tool` wrapper over [`crate::search::run_router_search`]
//! so agents can call the same pipeline the CLI uses.

#[path = "search_router_execute.rs"]
mod execute;

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

use super::{Tool, ToolResult};
use crate::provider::ProviderRegistry;

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
        execute::run(&self.registry, args).await
    }
}
