//! status for Kubernetes tool execution.
use super::*;
impl K8sTool {
    pub(crate) async fn exec_status(manager: &K8sManager, params: K8sInput) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let status = target.status().await;
        let output = serde_json::to_string_pretty(&json!({
            "in_cluster": status.in_cluster,
            "namespace": status.namespace,
            "pod_name": status.pod_name,
            "deployment_name": status.deployment_name,
            "replicas": status.replicas,
            "available_replicas": status.available_replicas,
        }))?;
        Ok(ToolResult::success(output))
    }
}
