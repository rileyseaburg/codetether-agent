//! Goal edits preserve runtime-injected session identity and native checks.

use super::{Host, context};
use anyhow::{Result, anyhow};
use codetether_goal_core::GoalEdit;
use codetether_tool_core::ToolResult;
use serde_json::Value;

/// Apply only within the invoking session; no caller-supplied target is exposed.
/// # Arguments
/// * `input` - Revision-bound edit and runtime-injected context.
/// # Returns
/// Updated state JSON or a model-visible refusal.
/// # Errors
/// Returns malformed-input, missing-session or serialization errors.
/// # Examples
/// ```no_run
/// async fn edit<H: codetether_goal_tools::Host>(input: serde_json::Value) -> anyhow::Result<()> {
///     let _result = codetether_goal_tools::edit::<H>(input).await?;
///     Ok(())
/// }
/// ```
pub async fn run<H: Host>(mut input: Value) -> Result<ToolResult> {
    if let Some(refused) = codetether_session_task::budget_refusal(&input) {
        return Ok(refused);
    }
    let object = input
        .as_object_mut()
        .ok_or_else(|| anyhow!("expected object"))?;
    let session_id = context::session_id(
        object
            .get("__ct_session_id")
            .and_then(Value::as_str)
            .map(str::to_owned),
    )?;
    object.retain(|key, _| !key.starts_with("__ct_"));
    let edit: GoalEdit = serde_json::from_value(input)?;
    match H::update(&session_id, edit).await {
        Ok(state) => Ok(ToolResult::success(serde_json::to_string(&state)?)),
        Err(error) => Ok(ToolResult::error(error.to_string())),
    }
}
