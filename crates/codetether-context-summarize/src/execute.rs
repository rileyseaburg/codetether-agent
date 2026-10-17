//! Tool trait implementation for `context_summarize`.

use super::schema;
use super::tool_struct::ContextSummarizeTool;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;

#[async_trait]
impl<S: crate::SummarySource> Tool for ContextSummarizeTool<S> {
    fn id(&self) -> &str {
        "context_summarize"
    }
    fn name(&self) -> &str {
        "ContextSummarize"
    }
    fn description(&self) -> &str {
        "Get or produce a cached summary for a turn range."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        super::run::run(self, args).await
    }
}
