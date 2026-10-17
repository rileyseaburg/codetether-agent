//! execute for relay coordination.
use super::*;
impl RelayAutoChatTool {
    pub(super) async fn dispatch(&self, params: Value) -> Result<ToolResult> {
        let action = match action::parse(&params) {
            Ok(action) => action,
            Err(result) => return Ok(result),
        };
        let params::Params {
            relay_id,
            target_agent,
            message,
            context,
            okr_id,
            task,
        } = params::Params::parse(&params);
        match action.as_str() {
            "init" => self.init_relay(relay_id, task, context, okr_id).await,
            "delegate" => {
                self.delegate_task(relay_id, target_agent, message, context, okr_id)
                    .await
            }
            "handoff" => {
                self.handoff_context(relay_id, target_agent, message, context)
                    .await
            }
            "status" => self.get_status(relay_id).await,
            "list_agents" => self.list_agents(relay_id).await,
            "complete" => self.complete_relay(relay_id).await,
            _ => Ok(ToolResult::structured_error(
                "INVALID_ACTION",
                "relay_autochat",
                &format!(
                    "Unknown action: '{action}'. Valid actions: init, delegate, handoff, status, list_agents, complete"
                ),
                None,
                Some(json!({
                    "action": "init",
                    "task": "description of the relay task"
                })),
            )),
        }
    }
}
