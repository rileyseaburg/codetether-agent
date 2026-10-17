//! RLM tool identity, schema and execution entrypoint.
use crate::{Host, RlmTool, execute, schema};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<H: Host> Tool for RlmTool<H> {
    fn id(&self) -> &str {
        "rlm"
    }

    fn name(&self) -> &str {
        "RLM"
    }

    fn description(&self) -> &str {
        schema::DESCRIPTION
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        execute::run(self, args).await
    }
}
