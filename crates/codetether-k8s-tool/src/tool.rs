//! tool for Kubernetes tool execution.
use super::*;
#[async_trait]
impl Tool for K8sTool {
    fn id(&self) -> &str {
        "kubernetes"
    }

    fn name(&self) -> &str {
        "kubernetes"
    }

    fn description(&self) -> &str {
        "Manage Kubernetes resources: check cluster status, list pods, scale deployments, trigger rolling restarts, spawn/delete sub-agent pods, fetch pod logs, and view recent management actions. Works both in-cluster and with local kubeconfig."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let params: K8sInput = serde_json::from_value(input)?;
        let manager = self.get_manager().await;

        if !manager.is_available() {
            return Ok(ToolResult::error(
                "Kubernetes is not available. Ensure the agent is running inside a cluster or KUBECONFIG is set.",
            ));
        }

        match params.action.clone() {
            K8sAction::Status => Self::exec_status(manager, params).await,
            K8sAction::ListPods => Self::exec_list_pods(manager, params).await,
            K8sAction::Scale => Self::exec_scale(manager, params).await,
            K8sAction::RollingRestart => Self::exec_rolling_restart(manager, params).await,
            K8sAction::SpawnPod => Self::exec_spawn_pod(manager, params).await,
            K8sAction::DeletePod => Self::exec_delete_pod(manager, params).await,
            K8sAction::PodState => Self::exec_pod_state(manager, params).await,
            K8sAction::Logs => Self::exec_logs(manager, params).await,
            K8sAction::RecentActions => Self::exec_recent_actions(manager).await,
        }
    }
}
