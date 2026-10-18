//! Construct explicit confirmation inputs from supported edit requests.
use crate::normalize_tool_call_for_execution;
use serde_json::Value;
use std::collections::HashMap;

/// Build a confirmation-apply request from a pending edit/multiedit tool call.
/// Returns `Some((confirm_tool_name, confirm_input))` if the tool call can be
/// auto-confirmed, or `None` if it is not a confirmable edit tool.
/// # Arguments
/// Receives a tool name, parsed input and optional result metadata.
/// # Returns
/// A supported confirm-tool request with confirm set to true, or None.
/// # Examples
/// ```
/// let request = codetether_edit_policy::build_pending_confirmation_apply_request("edit", &serde_json::json!({}), None).unwrap();
/// assert_eq!(request.0, "confirm_edit");
/// assert_eq!(request.1["confirm"], true);
/// ```
pub fn build_pending_confirmation_apply_request(
    tool_name: &str,
    tool_input: &Value,
    _tool_metadata: Option<&HashMap<String, Value>>,
) -> Option<(String, Value)> {
    let confirm_tool_name = match tool_name {
        "edit" | "confirm_edit" | "advanced_edit" => "confirm_edit",
        "multiedit" | "confirm_multiedit" => "confirm_multiedit",
        _ => return None,
    };

    let (_, mut normalized) = normalize_tool_call_for_execution(tool_name, tool_input);
    if let Some(obj) = normalized.as_object_mut() {
        obj.insert("confirm".to_string(), Value::Bool(true));
    }

    Some((confirm_tool_name.to_string(), normalized))
}
