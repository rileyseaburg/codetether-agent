//! pod state for Kubernetes tool execution.
use super::*;
impl K8sTool {
    pub(crate) async fn exec_pod_state(
        manager: &K8sManager,
        params: K8sInput,
    ) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let subagent_id = params
            .subagent_id
            .as_deref()
            .ok_or_else(|| anyhow!("'subagent_id' is required for the pod_state action"))?;

        let state = target
            .get_subagent_pod_state(subagent_id)
            .await?
            .ok_or_else(|| anyhow!("Pod for sub-agent '{}' not found", subagent_id))?;

        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "pod_name": state.pod_name,
            "phase": state.phase,
            "ready": state.ready,
            "terminated": state.terminated,
            "exit_code": state.exit_code,
            "reason": state.reason,
            "restart_count": state.restart_count,
        }))?))
    }
}
