use std::collections::HashMap;

use serde_json::{Value, json};

use crate::session::helper::tool_policy::ToolTuple;
use crate::tool::ToolResult;

pub(super) fn tuple(result: ToolResult) -> ToolTuple {
    (result.output, result.success, Some(result.metadata))
}

pub(super) fn denied(tool: &str, approval_id: &str, reason: Option<&str>) -> ToolTuple {
    let feedback = reason.map(str::trim).filter(|text| !text.is_empty());
    let message = match feedback {
        Some(reason) => format!(
            "Tool execution was denied by the user: {reason}. Nothing ran. \
             Follow this feedback. This exact request is blocked for the rest of the current turn."
        ),
        None => "Tool execution was denied by the user. Nothing ran. This exact request is \
                 blocked for the rest of the current turn; choose a different approach or ask the user."
            .to_string(),
    };
    let result = ToolResult::structured_error("TOOL_APPROVAL_DENIED", tool, &message, None, None)
        .with_metadata("approval_request_id", json!(approval_id))
        .with_metadata("retry_same_call", json!(false));
    tuple(result)
}

pub(super) fn text(map: &HashMap<String, Value>, key: &str) -> Option<String> {
    map.get(key).and_then(Value::as_str).map(str::to_string)
}
