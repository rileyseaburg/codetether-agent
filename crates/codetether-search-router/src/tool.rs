//! Tool metadata and typed registry/memory bindings.
use crate::{Registry, SearchTool};
use anyhow::Result;
use async_trait::async_trait;
use codetether_memory_tool::AutoEmbedder;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<R: Registry, B: AutoEmbedder> Tool for SearchTool<R, B> {
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
        super::schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        super::execute::run::<R, B>(&self.registry, args).await
    }
}
