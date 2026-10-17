//! Dispatch from first-class tools into the existing agent runtime.

use crate::{Host, RuntimeContext};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::{Map, Value};

/// Forward a payload with the original trusted runtime fields.
/// # Arguments
/// * `context` — Trusted parent runtime context.
/// * `payload` — Legacy agent operation arguments.
/// # Returns
/// The existing agent engine's tool result.
/// # Errors
/// Returns the engine's error without rewriting it.
/// # Examples
/// The lifecycle tools call this after preparing their action-specific payload.
pub async fn execute<H: Host>(
    context: &RuntimeContext,
    mut payload: Map<String, Value>,
) -> Result<ToolResult> {
    context.inject(&mut payload);
    H::execute(Value::Object(payload)).await
}
