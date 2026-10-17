//! Lease-gated pod deletion and action recording.

use crate::{DeployAction, K8sManager};
use anyhow::{Result, anyhow};
use k8s_openapi::api::core::v1::Pod;
use kube::Api;
impl K8sManager {
    /// Delete a sub-agent pod.
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
    /// let action = manager.delete_subagent_pod("task").await?;
    /// assert!(action.success);
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn delete_subagent_pod(&self, subagent_id: &str) -> Result<DeployAction> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available"))?;

        let pod_name = Self::subagent_pod_name(subagent_id);
        let claim = self
            .coordinator
            .acquire(
                client.clone(),
                &self.namespace,
                "Pod",
                &pod_name,
                "delete-subagent",
            )
            .await?;
        let pods: Api<Pod> = Api::namespaced(client.clone(), &self.namespace);

        super::delete_request::delete(&pods, &pod_name, subagent_id).await?;
        let action = super::actions::deleted(subagent_id, &pod_name);
        self.record_action(action.clone()).await;
        claim.release().await;
        Ok(action)
    }
}
