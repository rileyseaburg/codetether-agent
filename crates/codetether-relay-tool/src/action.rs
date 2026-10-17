//! action for relay coordination.
use super::*;
pub(super) fn parse(params: &Value) -> std::result::Result<String, ToolResult> {
    let action = match params.get("action").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => {
            return Err(ToolResult::structured_error(
                "MISSING_FIELD",
                "relay_autochat",
                "action is required. Valid actions: init, delegate, handoff, status, list_agents, complete",
                Some(vec!["action"]),
                Some(json!({
                    "action": "init",
                    "task": "description of the relay task"
                })),
            ));
        }
    };

    Ok(action)
}
