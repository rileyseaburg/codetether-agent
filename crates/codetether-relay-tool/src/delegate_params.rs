//! delegate params for relay coordination.
use super::*;
pub(super) fn validate(
    relay_id: Option<String>,
    target_agent: Option<String>,
) -> std::result::Result<(String, String), ToolResult> {
    let relay_id = match relay_id {
        Some(id) => id,
        None => {
            return Err(ToolResult::structured_error(
                "MISSING_FIELD",
                "relay_autochat",
                "relay_id is required for delegate action",
                Some(vec!["relay_id"]),
                Some(
                    json!({"action": "delegate", "relay_id": "relay-xxx", "target_agent": "agent-name", "message": "task description"}),
                ),
            ));
        }
    };
    let target_agent = match target_agent {
        Some(a) => a,
        None => {
            return Err(ToolResult::structured_error(
                "MISSING_FIELD",
                "relay_autochat",
                "target_agent is required for delegate action",
                Some(vec!["target_agent"]),
                Some(
                    json!({"action": "delegate", "relay_id": relay_id, "target_agent": "agent-name", "message": "task description"}),
                ),
            ));
        }
    };
    Ok((relay_id, target_agent))
}
