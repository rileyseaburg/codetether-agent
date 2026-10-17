//! Dispatch the optional interruption through trusted legacy forwarding.
use crate::{MessagingHost, RuntimeContext, legacy};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::{Map, json};

pub(super) async fn execute<H: MessagingHost>(
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
    legacy::execute::<H>(context, payload).await
}
