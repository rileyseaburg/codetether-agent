//! rolling restart for Kubernetes tool execution.
use super::*;
impl K8sTool {
    pub(crate) async fn exec_rolling_restart(
        manager: &K8sManager,
        params: K8sInput,
    ) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let action = target.rolling_restart().await?;
        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "action": action.action,
            "success": action.success,
            "message": action.message,
            "timestamp": action.timestamp,
        }))?))
    }
}
