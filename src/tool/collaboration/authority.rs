//! Claimed authority required by collaboration side-effect dispatch.

use crate::tool::ToolResult;
use serde_json::Value;

pub(super) struct Claimed(());

pub(super) async fn claim(tool: &str, input: &Value) -> Result<Claimed, ToolResult> {
    match crate::tool::orchestration_gate::blocked(tool, input).await {
        Some(blocked) => Err(blocked),
        None => Ok(Claimed(())),
    }
}