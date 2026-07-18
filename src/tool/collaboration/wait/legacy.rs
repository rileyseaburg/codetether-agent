//! Hidden compatibility path for V1 target-based waits.

use super::{subscriptions, targets};
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::{Map, Value, json};
use tokio::time::Instant;

pub(super) async fn execute(
    owner: &str,
    requested: &[String],
    deadline: Instant,
) -> Result<ToolResult> {
    let (initial, receivers) = targets::resolve(owner, requested)?;
    if !initial.is_empty() {
        return Ok(result(initial, false));
    }
    let settled = subscriptions::first_final(receivers, deadline).await;
    let timed_out = settled.is_empty();
    Ok(result(settled, timed_out))
}

fn result(status: Map<String, Value>, timed_out: bool) -> ToolResult {
    ToolResult::success(json!({"status":status, "timed_out":timed_out}).to_string())
}
