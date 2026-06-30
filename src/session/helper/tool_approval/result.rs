use std::collections::HashMap;

use serde_json::{Value, json};

use crate::session::helper::tool_policy::ToolTuple;
use crate::tool::ToolResult;

pub(super) fn tuple(result: ToolResult) -> ToolTuple {
    (result.output, result.success, Some(result.metadata))
}

pub(super) fn denied(tool: &str, approval_id: &str, reason: Option<&str>) -> ToolTuple {
    let message = match reason {
        Some(reason) => format!("Tool execution was denied by the user: {reason}"),
        None => "Tool execution was denied by the user.".to_string(),
    };
    let result = ToolResult::structured_error("TOOL_APPROVAL_DENIED", tool, &message, None, None)
        .with_metadata("approval_request_id", json!(approval_id));
    tuple(result)
}

pub(super) fn text(map: &HashMap<String, Value>, key: &str) -> Option<String> {
    map.get(key).and_then(Value::as_str).map(str::to_string)
}
