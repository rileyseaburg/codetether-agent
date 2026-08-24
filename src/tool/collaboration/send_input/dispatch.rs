//! Interrupt and durable submission dispatch for `send_input`.

use super::input::Prepared;
use crate::tool::collaboration::{authority::Claimed, context::RuntimeContext, legacy};
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::{Map, json};

pub(super) async fn execute(
    authority: Claimed,
    context: RuntimeContext,
    target: String,
    input: Prepared,
    interrupt: bool,
) -> Result<ToolResult> {
    if let Some(result) = super::super::ensure::ready(&context, &target).await? {
        return Ok(result);
    }
    if interrupt {
        let result = action(&authority, &context, "interrupt", &target, None).await?;
        if !result.success {
            return Ok(result);
        }
    }
    let mut payload = json!({
        "action":"message", "name":target, "message":input.message,
        "detach":true, "__ct_message_images":input.images
    })
    .as_object()
    .cloned()
    .expect("object payload");
    context.inject(&mut payload);
    crate::tool::agent::AgentTool::execute_authorized(serde_json::Value::Object(payload))
        .await
}

async fn action(
    authority: &Claimed,
    context: &RuntimeContext,
    action: &str,
    target: &str,
    message: Option<String>,
) -> Result<ToolResult> {
    let payload: Map<String, serde_json::Value> = json!({
        "action":action, "name":target, "message":message
    })
    .as_object()
    .cloned()
    .expect("object payload");
    legacy::execute(authority, context, payload).await
}