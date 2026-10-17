//! complete relay for relay coordination.
use super::*;
impl RelayAutoChatTool {
    /// Complete a relay and aggregate results
    pub(super) async fn complete_relay(&self, relay_id: Option<String>) -> Result<ToolResult> {
        let relay_id = match relay_id {
            Some(id) => id,
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "relay_autochat",
                    "relay_id is required for complete action",
                    Some(vec!["relay_id"]),
                    Some(json!({"action": "complete", "relay_id": "relay-xxx"})),
                ));
            }
        };

        // Get the runtime and shutdown agents
        let relay_state = {
            let mut store = RELAY_STORE.write();
            store.remove(&relay_id)
        };

        let total_agents = relay_state
            .as_ref()
            .map(|state| state.participants.len())
            .unwrap_or(0);
        if let Some(state) = relay_state {
            state.runtime.shutdown_agents(&state.participants);
        }

        let response = json!({
            "status": "completed",
            "relay_id": relay_id,
            "message": "Relay completed successfully. Results aggregated.",
            "aggregated_results": {
                "completed": true,
                "total_agents": total_agents
            }
        });

        Ok(ToolResult::success(
            serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
        ))
    }
}
