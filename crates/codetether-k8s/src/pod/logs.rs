//! Kubernetes manager logs.

use crate::K8sManager;
use anyhow::{Context, Result, anyhow};
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, api::LogParams};
impl K8sManager {
    /// Fetch recent logs for a sub-agent pod.
    ///
    /// # Errors
    /// Returns an error if the client is unavailable, mutation ownership cannot be acquired,
    /// or the required Kubernetes API operation fails.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_k8s::{K8sManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = K8sManager::new().await;
    /// let logs: String = manager.subagent_logs("task", 100).await?;
    /// let _ = logs;
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn subagent_logs(&self, subagent_id: &str, tail_lines: i64) -> Result<String> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available"))?;
        let pods: Api<Pod> = Api::namespaced(client.clone(), &self.namespace);
        let pod_name = Self::subagent_pod_name(subagent_id);
        let params = LogParams {
            tail_lines: Some(tail_lines),
            ..LogParams::default()
        };
        pods.logs(&pod_name, &params)
            .await
            .with_context(|| format!("Failed to fetch logs for sub-agent pod {pod_name}"))
    }
}
