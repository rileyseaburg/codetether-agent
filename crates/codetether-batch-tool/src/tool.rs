//! Tool registry metadata and batch execution entry point.
use super::{BatchTool, Registry};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<R: Registry> Tool for BatchTool<R> {
    fn id(&self) -> &str {
        "batch"
    }

    fn name(&self) -> &str {
        "Batch Execute"
    }

    fn description(&self) -> &str {
        "Execute at most three short independent tool calls in parallel. Do not use for large file writes, long shell scripts, or broad search batches; call tools individually instead."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        super::execute::execute(self, params).await
    }
}
