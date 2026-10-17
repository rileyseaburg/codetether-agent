//! list agents for relay coordination.
use super::*;
impl RelayAutoChatTool {
    /// List agents in a relay
    pub(super) async fn list_agents(&self, relay_id: Option<String>) -> Result<ToolResult> {
        let relay_id = match relay_id {
            Some(id) => id,
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "relay_autochat",
                    "relay_id is required for list_agents action",
                    Some(vec!["relay_id"]),
                    Some(json!({"action": "list_agents", "relay_id": "relay-xxx"})),
                ));
            }
        };

        let participants = {
            let store = RELAY_STORE.read();
            store.get(&relay_id).map(|state| state.participants.clone())
        };

        if let Some(participants) = participants {
            let agents: Vec<Value> = participants
                .iter()
                .map(|name| json!({ "name": name }))
                .collect();

            let response = json!({
                "relay_id": relay_id,
                "agents": agents,
                "count": agents.len()
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
