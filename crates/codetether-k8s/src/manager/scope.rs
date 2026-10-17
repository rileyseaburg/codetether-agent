//! Kubernetes manager scope.

use crate::K8sManager;

impl K8sManager {
    /// Whether K8s integration is available.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_k8s::{K8sManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = K8sManager::new().await;
    /// let available: bool = manager.is_available();
    /// assert_eq!(available, manager.is_available());
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub fn is_available(&self) -> bool {
        self.client.is_some()
    }

    /// Return a clone scoped to an explicit namespace and/or deployment.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_k8s::{K8sManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = K8sManager::new().await;
    /// let scoped = manager.scoped(Some("jobs"), Some("workers"));
    /// assert!(format!("{scoped:?}").contains("jobs"));
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub fn scoped(&self, namespace: Option<&str>, deployment_name: Option<&str>) -> Self {
        let mut scoped = self.clone();
        if let Some(namespace) = namespace.filter(|value| !value.trim().is_empty()) {
            scoped.namespace = namespace.to_string();
        }
        if let Some(deployment_name) = deployment_name.filter(|value| !value.trim().is_empty()) {
            scoped.deployment_name = Some(deployment_name.to_string());
        }
        scoped
    }
}
