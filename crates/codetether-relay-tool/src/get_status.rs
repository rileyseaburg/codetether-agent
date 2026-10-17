//! get status for relay coordination.
use super::*;
impl RelayAutoChatTool {
    /// Get status of a relay
    pub(super) async fn get_status(&self, relay_id: Option<String>) -> Result<ToolResult> {
        let relay_id = match relay_id {
            Some(id) => id,
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "relay_autochat",
                    "relay_id is required for status action",
                    Some(vec!["relay_id"]),
                    Some(json!({"action": "status", "relay_id": "relay-xxx"})),
                ));
            }
        };

        let store = RELAY_STORE.read();

        if store.contains_key(&relay_id) {
            let response = json!({
                "status": "active",
                "relay_id": relay_id,
                "message": "Relay is active"
            });

            Ok(ToolResult::success(
                serde_json::to_string_pretty(&response)
                    .unwrap_or_else(|_| format!("{:?}", response)),
            ))
        } else {
            Ok(ToolResult::error(format!("Relay not found: {}", relay_id)))
        }
    }
}
