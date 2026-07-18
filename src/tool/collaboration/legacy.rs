//! Dispatch from first-class tools into the existing agent runtime.

use super::context::RuntimeContext;
use crate::tool::{Tool, ToolResult, agent::AgentTool};
use anyhow::Result;
use serde_json::{Map, Value};

pub(super) async fn execute(
    context: &RuntimeContext,
    mut payload: Map<String, Value>,
) -> Result<ToolResult> {
    context.inject(&mut payload);
    AgentTool::new().execute(Value::Object(payload)).await
}
