//! Tool metadata and generic policy-aware execution entry point.
use crate::{Policy, Undo};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<P: Policy> Tool for Undo<P> {
    fn id(&self) -> &str {
        "undo"
    }

    fn name(&self) -> &str {
        "Undo"
    }

    fn description(&self) -> &str {
        "Undo the last AI-generated changes by reverting git commits"
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        super::execute::run::<P>(input).await
    }
}
