//! Ralph tool identity, schema and action entrypoint.

use crate::{Host, RalphTool};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<H: Host> Tool for RalphTool<H> {
    fn id(&self) -> &str {
        "ralph"
    }

    fn name(&self) -> &str {
        "Ralph Agent"
    }
    fn description(&self) -> &str {
        include_str!("description.md")
    }
    fn parameters(&self) -> Value {
        crate::schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        crate::dispatch::execute(self, params).await
    }
}
