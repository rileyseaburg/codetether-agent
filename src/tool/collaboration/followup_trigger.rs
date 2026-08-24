//! Approved idle-child turn trigger.

use super::Args;
use super::super::{authority::Claimed, legacy};
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::json;

pub(super) async fn run(args: Args, authority: Claimed) -> Result<ToolResult> {
    let mut result = legacy::execute(
        &authority,
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