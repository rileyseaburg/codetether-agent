//! spawn pod for Kubernetes tool execution.
use super::*;
impl K8sTool {
    pub(crate) async fn exec_spawn_pod(
        manager: &K8sManager,
        params: K8sInput,
    ) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let subagent_id = params
            .subagent_id
            .as_deref()
            .ok_or_else(|| anyhow!("'subagent_id' is required for the spawn_pod action"))?;

        let spec = codetether_k8s::SubagentPodSpec {
            image: params.image,
            env_vars: params.env_vars.unwrap_or_default(),
            labels: params.labels.unwrap_or_default(),
            command: params.command,
            args: params.args,
        };

        let action = target
            .spawn_subagent_pod_with_spec(subagent_id, spec)
            .await?;
        Ok(ToolResult::success(serde_json::to_string_pretty(&json!({
            "action": action.action,
            "success": action.success,
            "message": action.message,
            "timestamp": action.timestamp,
        }))?))
    }
}
