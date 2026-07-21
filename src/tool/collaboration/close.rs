//! First-class `close_agent` tool.

use super::{context::RuntimeContext, legacy};
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) struct CloseAgentTool;

#[derive(Deserialize)]
struct Args {
    target: String,
    #[serde(flatten)]
    context: RuntimeContext,
}

#[async_trait]
impl Tool for CloseAgentTool {
    fn id(&self) -> &str {
        "close_agent"
    }
    fn name(&self) -> &str {
        "Close Agent"
    }
    fn description(&self) -> &str {
        "Close a sub-agent subtree, preserving durable transcripts and queued work for resume."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{"target":{"type":"string"}},
            "required":["target"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        legacy::execute(
            &args.context,
            json!({"action":"close", "name":args.target})
                .as_object()
                .cloned()
                .unwrap_or_default(),
        )
        .await
    }
}
