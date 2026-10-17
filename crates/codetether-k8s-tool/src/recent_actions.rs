//! recent actions for Kubernetes tool execution.
use super::*;
impl K8sTool {
    pub(crate) async fn exec_recent_actions(manager: &K8sManager) -> Result<ToolResult> {
        let actions = manager.recent_actions(20).await;
        let actions_json: Vec<Value> = actions
            .iter()
            .map(|a| {
                json!({
                    "action": a.action,
                    "success": a.success,
                    "message": a.message,
                    "timestamp": a.timestamp,
                })
            })
            .collect();

        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "actions": actions_json,
            "count": actions_json.len(),
        }))?))
    }
}
