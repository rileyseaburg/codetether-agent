//! First-class `resume_agent` tool.

use super::{context::RuntimeContext, legacy};
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) struct ResumeAgentTool;

#[derive(Deserialize)]
struct Args {
    id: String,
    #[serde(flatten)]
    context: RuntimeContext,
}

#[async_trait]
impl Tool for ResumeAgentTool {
    fn id(&self) -> &str { "resume_agent" }
    fn name(&self) -> &str { "Resume Agent" }
    fn description(&self) -> &str {
        "Resume a closed durable child thread by its session ID."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{"id":{"type":"string"}},
            "required":["id"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        legacy::execute(&args.context, json!({"action":"resume", "name":args.id})
            .as_object().cloned().unwrap_or_default()).await
    }
}
