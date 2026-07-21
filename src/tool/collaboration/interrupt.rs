//! First-class `interrupt_agent` tool.

use super::{context::RuntimeContext, legacy};
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) struct InterruptAgentTool;

#[derive(Deserialize)]
struct Args {
    target: String,
    #[serde(flatten)]
    context: RuntimeContext,
}

#[async_trait]
impl Tool for InterruptAgentTool {
    fn id(&self) -> &str {
        "interrupt_agent"
    }
    fn name(&self) -> &str {
        "Interrupt Agent"
    }
    fn description(&self) -> &str {
        "Interrupt an active sub-agent turn without deleting its persisted session."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{"target":{"type":"string"}},
            "required":["target"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        legacy::execute(
            &args.context,
            json!({
                "action":"interrupt", "name":args.target
            })
            .as_object()
            .cloned()
            .unwrap_or_default(),
        )
        .await
    }
}
