//! Ensure readiness, optionally interrupt, then submit durable input.
use crate::{MessagingHost, RuntimeContext};
use anyhow::Result;
use codetether_collaboration_input::Prepared;
use codetether_tool_core::ToolResult;
use serde_json::json;

pub(super) async fn execute<H: MessagingHost>(
    context: RuntimeContext,
    target: String,
    input: Prepared,
    interrupt: bool,
) -> Result<ToolResult> {
    if let Some(result) = crate::ensure::ready::<H>(&context, &target).await? {
        return Ok(result);
    }
    if interrupt {
        let result = super::action::execute::<H>(&context, "interrupt", &target, None).await?;
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
    H::execute(serde_json::Value::Object(payload)).await
}
