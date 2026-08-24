//! Dispatch from first-class tools into the existing agent runtime.

use super::authority::Claimed;
use super::context::RuntimeContext;
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::{Map, Value};

pub(super) async fn execute(
    _authority: &Claimed,
    context: &RuntimeContext,
    mut payload: Map<String, Value>,
) -> Result<ToolResult> {
    context.inject(&mut payload);
    crate::tool::agent::AgentTool::execute_authorized(Value::Object(payload)).await
}