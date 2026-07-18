//! Codex-compatible queue-only `send_message` tool.

use super::context::RuntimeContext;
use crate::tool::{Tool, ToolResult};
use anyhow::{Result, bail};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

pub(super) struct SendMessageTool;

#[derive(Deserialize)]
struct Args {
    target: String,
    message: String,
    #[serde(flatten)]
    context: RuntimeContext,
}

#[async_trait]
impl Tool for SendMessageTool {
    fn id(&self) -> &str { "send_message" }
    fn name(&self) -> &str { "Send Message" }
    fn description(&self) -> &str {
        "Deliver context to a child promptly without starting an idle child turn."
    }
    fn parameters(&self) -> Value {
        json!({"type":"object","properties":{
            "target":{"type":"string"}, "message":{"type":"string"}
        },"required":["target","message"]})
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let args: Args = serde_json::from_value(input)?;
        if args.message.trim().is_empty() { bail!("Empty message can't be sent to an agent"); }
        if let Some(result) = super::ensure::ready(&args.context, &args.target).await? {
            return Ok(result);
        }
        crate::tool::agent::communication::queue_only(
            &args.target,
            args.context.session_id.as_deref(),
            args.message,
        )
        .await
    }
}
