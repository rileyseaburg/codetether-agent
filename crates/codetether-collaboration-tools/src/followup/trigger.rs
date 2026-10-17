//! Launch an idle child turn through the existing detached-message operation.
use super::args::Args;
use crate::{MessagingHost, legacy};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::json;

pub(super) async fn run<H: MessagingHost>(args: Args) -> Result<ToolResult> {
    let mut result = legacy::execute::<H>(
        &args.context,
        json!({
            "action":"message", "name":args.target,
            "message":args.message, "detach":true
        })
        .as_object()
        .cloned()
        .expect("object payload"),
    )
    .await?;
    if result.success {
        result.output.clear();
    }
    Ok(result)
}
