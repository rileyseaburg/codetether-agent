//! handoff context for relay coordination.
use super::*;
impl RelayAutoChatTool {
    /// Hand off context between agents
    pub(super) async fn handoff_context(
        &self,
        relay_id: Option<String>,
        target_agent: Option<String>,
        message: Option<String>,
        context: Option<Value>,
    ) -> Result<ToolResult> {
        let (relay_id, target_agent) = match handoff_params::validate(relay_id, target_agent) {
            Ok(fields) => fields,
            Err(result) => return Ok(result),
        };
        let message = message.unwrap_or_else(|| "Context handoff".to_string());

        let store = RELAY_STORE.read();
        let runtime = match store.get(&relay_id) {
            Some(state) => Arc::clone(&state.runtime),
            None => {
                return Ok(ToolResult::structured_error(
                    "NOT_FOUND",
                    "relay_autochat",
                    &format!(
                        "Relay not found: {relay_id}. Use 'init' action to create a relay first."
                    ),
                    None,
                    Some(json!({"action": "init", "task": "description of the relay task"})),
                ));
            }
        };
        // need to drop the lock before await
        drop(store);

        // Build context payload
        let context_msg = handoff_message::build(message, context);

        // Send handoff
        runtime.send_handoff("previous_agent", &target_agent, &context_msg);
        Self::remember_participant(&relay_id, &target_agent);

        let response = json!({
            "status": "handoff_complete",
            "relay_id": relay_id,
            "target_agent": target_agent,
            "message": "Context successfully handed off to target agent"
        });

        Ok(ToolResult::success(
            serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
        ))
    }
}
