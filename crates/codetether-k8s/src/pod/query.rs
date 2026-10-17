//! Read a sub-agent pod and map API failures to absent state.

use crate::{K8sManager, SubagentPodState};
use anyhow::{Result, anyhow};
use k8s_openapi::api::core::v1::Pod;
use kube::Api;
impl K8sManager {
    /// Get pod state for a sub-agent.
    ///
    /// API lookup failures and missing pods yield `Ok(None)`.
    ///
    /// # Errors
    /// Returns an error if no Kubernetes client is available.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_k8s::{K8sManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = K8sManager::new().await;
    /// if let Some(state) = manager.get_subagent_pod_state("task").await? {
    ///     assert_eq!(state.pod_name, K8sManager::subagent_pod_name("task"));
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn get_subagent_pod_state(
        &self,
        subagent_id: &str,
    ) -> Result<Option<SubagentPodState>> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available"))?;

        let pods: Api<Pod> = Api::namespaced(client.clone(), &self.namespace);
        let pod_name = Self::subagent_pod_name(subagent_id);

        let pod = match pods.get_opt(&pod_name).await {
            Ok(p) => p,
            Err(e) => {
                tracing::warn!(pod = %pod_name, error = %e, "Failed to fetch sub-agent pod state");
                return Ok(None);
            }
        };

        let Some(pod) = pod else {
            return Ok(None);
        };

        Ok(Some(super::projection::state(pod_name, &pod)))
    }
}
