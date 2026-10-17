//! list pods for Kubernetes tool execution.
use super::*;
impl K8sTool {
    pub(crate) async fn exec_list_pods(
        manager: &K8sManager,
        params: K8sInput,
    ) -> Result<ToolResult> {
        let target = manager.scoped(params.namespace.as_deref(), params.deployment.as_deref());
        let pods = target
            .list_pods_with_selector(params.label_selector.as_deref())
            .await?;
        let pods_json: Vec<Value> = pods
            .iter()
            .map(|p| {
                json!({
                    "name": p.name,
                    "phase": p.phase,
                    "ready": p.ready,
                    "start_time": p.start_time,
                })
            })
            .collect();

        let output = json!({
            "namespace": target.status().await.namespace,
            "pods": pods_json,
            "count": pods_json.len(),
        });
        Ok(ToolResult::success(serde_json::to_string_pretty(&output)?))
    }
}
