//! scale for Kubernetes tool execution.
use super::*;
impl K8sTool {
    pub(crate) async fn exec_scale(manager: &K8sManager, params: K8sInput) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let replicas = params
            .replicas
            .ok_or_else(|| anyhow!("'replicas' is required for the scale action"))?;
        let action = target.scale(replicas).await?;
        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "action": action.action,
            "success": action.success,
            "message": action.message,
            "timestamp": action.timestamp,
        }))?))
    }
}
