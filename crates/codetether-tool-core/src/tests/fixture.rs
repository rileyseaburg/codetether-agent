//! Side-effect-free tool for testing interface adapters.
use crate::{Tool, ToolResult};
use serde_json::{Value, json};

pub(super) struct Echo;

#[async_trait::async_trait]
impl Tool for Echo {
    fn id(&self) -> &str {
        "echo"
    }
    fn name(&self) -> &str {
        "Echo"
    }
    fn description(&self) -> &str {
        "Return the supplied input"
    }
    fn parameters(&self) -> Value {
        json!({"type":"object"})
    }
    async fn execute(&self, args: Value) -> anyhow::Result<ToolResult> {
        Ok(ToolResult::success(args.to_string()).with_metadata("input", args))
    }
}
