//! Model-facing metadata for context budget reporting.
use crate::{BudgetSource, ContextBudgetTool};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
#[async_trait]
impl<S: BudgetSource> Tool for ContextBudgetTool<S> {
    fn id(&self) -> &str {
        "context_budget"
    }

    fn name(&self) -> &str {
        "ContextBudget"
    }

    fn description(&self) -> &str {
        "Report the current context window budget: messages count, \
         cached summaries, page distribution, and remaining capacity. \
         Call this to understand how much room is left before the next \
         compression or reset. Returns L_working (current transcript \
         length) and L_effective (with summarization multiplier)."
    }

    fn parameters(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        super::execute::run::<S>(args).await
    }
}
