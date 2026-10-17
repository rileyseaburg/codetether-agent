//! logs for Kubernetes tool execution.
use super::*;
impl K8sTool {
    pub(crate) async fn exec_logs(manager: &K8sManager, params: K8sInput) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let subagent_id = params
            .subagent_id
            .as_deref()
            .ok_or_else(|| anyhow!("'subagent_id' is required for the logs action"))?;
        let logs = target.subagent_logs(subagent_id, params.tail_lines).await?;
        Ok(ToolResult::success(logs))
    }
}
