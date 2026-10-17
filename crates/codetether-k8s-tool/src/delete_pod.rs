//! delete pod for Kubernetes tool execution.
use super::*;
impl K8sTool {
    pub(crate) async fn exec_delete_pod(
        manager: &K8sManager,
        params: K8sInput,
    ) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let subagent_id = params
            .subagent_id
            .as_deref()
            .ok_or_else(|| anyhow!("'subagent_id' is required for the delete_pod action"))?;
        let action = target.delete_subagent_pod(subagent_id).await?;
        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "action": action.action,
            "success": action.success,
            "message": action.message,
            "timestamp": action.timestamp,
        }))?))
    }
}
