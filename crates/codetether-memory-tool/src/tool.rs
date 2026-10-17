//! Memory tool metadata and action dispatch.
use super::{AutoEmbedder, Runtime};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<B: AutoEmbedder> Tool for Runtime<B> {
    fn id(&self) -> &str {
        "memory"
    }

    fn name(&self) -> &str {
        "Memory"
    }

    fn description(&self) -> &str {
        "Store and retrieve persistent knowledge across sessions. Use 'save' to capture important insights, 'search' to find relevant memories, 'list' to see all entries, 'tags' to see available categories, or 'delete' to remove an entry."
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        // Initialize once per tool instance. Interactive memory operations use
        // the deterministic local embedding engine and never await providers.
        self.init().await.ok();

        let action = args["action"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("action is required"))?;

        match action {
            "save" => self.execute_save(args).await,
            "search" => self.execute_search(args).await,
            "get" => self.execute_get(args).await,
            "list" => self.execute_list(args).await,
            "tags" => self.execute_tags(args).await,
            "delete" => self.execute_delete(args).await,
            "stats" => self.execute_stats(args).await,
            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Use 'save', 'search', 'get', 'list', 'tags', 'delete', or 'stats'.",
                action
            ))),
        }
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
}
