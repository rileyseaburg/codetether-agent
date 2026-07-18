//! Codex-compatible `send_input` tool.

use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[path = "send_input/args.rs"]
mod args;
#[path = "send_input/dispatch.rs"]
mod dispatch;
#[path = "send_input/input.rs"]
mod input;
#[path = "send_input/item.rs"]
mod item;
#[path = "send_input/schema.rs"]
mod schema;

#[cfg(test)]
#[path = "send_input_tests.rs"]
mod tests;

pub(super) struct SendInputTool;

#[async_trait]
impl Tool for SendInputTool {
    fn id(&self) -> &str {
        "send_input"
    }
    fn name(&self) -> &str {
        "Send Input"
    }
    fn description(&self) -> &str {
        "Queue structured input on a child thread and return its durable submission ID."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, value: Value) -> Result<ToolResult> {
        let args: args::Args = serde_json::from_value(value)?;
        let prepared = input::prepare(args.message, args.items).await?;
        dispatch::execute(args.context, args.target, prepared, args.interrupt).await
    }
}
