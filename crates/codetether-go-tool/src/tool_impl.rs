//! Tool identity, schema and action dispatch.

use crate::{GoTool, Host};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<H: Host> Tool for GoTool<H> {
    fn id(&self) -> &str {
        "go"
    }

    fn name(&self) -> &str {
        "Go"
    }
    fn description(&self) -> &str {
        crate::description::DESCRIPTION
    }
    fn parameters(&self) -> Value {
        crate::schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        crate::dispatch::execute(self, params).await
    }
}
