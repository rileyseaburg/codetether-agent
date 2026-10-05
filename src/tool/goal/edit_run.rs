//! Goal edits preserve runtime-injected session identity and native checks.

use super::context;
use crate::session::tasks::{GoalEdit, control};
use crate::tool::ToolResult;
use anyhow::{Result, anyhow};
use serde_json::Value;

/// Apply only within the invoking session; no caller-supplied target is exposed.
pub(super) async fn run(mut input: Value) -> Result<ToolResult> {
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
    match control::update(&session_id, edit).await {
        Ok(state) => Ok(ToolResult::success(serde_json::to_string(&state)?)),
        Err(error) => Ok(ToolResult::error(error.to_string())),
    }
}
