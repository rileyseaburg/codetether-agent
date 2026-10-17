//! Kubernetes manager status.

use crate::K8sManager;
use crate::K8sStatus;
impl K8sManager {
    /// Get current status.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_k8s::{K8sManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = K8sManager::new().await;
    /// let status = manager.status().await;
    /// assert!(!status.namespace.is_empty());
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn status(&self) -> K8sStatus {
        let (replicas, available) = if let Some(ref client) = self.client {
            if let Some(ref dep_name) = self.deployment_name {
                self.get_deployment_replicas(client, dep_name).await
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        K8sStatus {
            in_cluster: self.client.is_some(),
            namespace: self.namespace.clone(),
            pod_name: self.pod_name.clone(),
            deployment_name: self.deployment_name.clone(),
            replicas,
            available_replicas: available,
        }
    }
}
