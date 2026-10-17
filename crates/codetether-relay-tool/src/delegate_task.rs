//! delegate task for relay coordination.
use super::*;
impl RelayAutoChatTool {
    /// Delegate a task to a target agent
    pub(super) async fn delegate_task(
        &self,
        relay_id: Option<String>,
        target_agent: Option<String>,
        message: Option<String>,
        context: Option<Value>,
        okr_id: Option<String>,
    ) -> Result<ToolResult> {
        let (relay_id, target_agent) = match delegate_params::validate(relay_id, target_agent) {
            Ok(fields) => fields,
            Err(result) => return Ok(result),
        };
        let message = message.unwrap_or_else(|| "New task assigned".to_string());

        let runtime = self.get_or_create_runtime(&relay_id).await?;

        // Build context payload if provided
        let context_msg = if let Some(ref ctx) = context {
            format!(
                "{}\n\nContext: {}",
                message,
                serde_json::to_string_pretty(ctx).unwrap_or_default()
            )
        } else {
            message.clone()
        };

        // Send the delegation message
        runtime.send_handoff("system", &target_agent, &context_msg);
        Self::remember_participant(&relay_id, &target_agent);

        let response = json!({
            "status": "delegated",
            "relay_id": relay_id,
            "target_agent": target_agent,
            "okr_id": okr_id,
            "message": message,
            "initial_results": {
                "task_assigned": true,
                "agent_notified": true
            }
        });

        let mut result = ToolResult::success(
            serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
        )
        .with_metadata("relay_id", json!(relay_id))
        .with_metadata("target_agent", json!(target_agent));
        if let Some(okr_id) = response.get("okr_id").and_then(|v| v.as_str()) {
            result = result.with_metadata("okr_id", json!(okr_id));
        }

        Ok(result)
    }
}
