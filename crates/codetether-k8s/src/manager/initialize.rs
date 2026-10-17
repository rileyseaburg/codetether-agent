//! Resolve manager identity and initialize an optional Kubernetes client.

use crate::{K8sManager, coordination};
use std::sync::Arc;
use tokio::sync::RwLock;
impl K8sManager {
    /// Attempt to initialize from in-cluster configuration.
    /// Returns a manager even if not running in K8s (with client = None).
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_k8s::{K8sManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = K8sManager::new().await;
    /// assert!(!format!("{manager:?}").is_empty());
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn new() -> Self {
        // rustls 0.23+ requires selecting a process-level crypto provider.
        // This path is exercised by unit tests (and library users) without
        // going through the binary's startup initialization.
        codetether_tls::ensure_rustls_crypto_provider();

        let namespace = std::env::var("CODETETHER_K8S_NAMESPACE")
            .or_else(|_| Self::read_namespace_file())
            .unwrap_or_else(|_| "default".to_string());

        let pod_name = std::env::var("HOSTNAME")
            .ok()
            .or_else(|| std::env::var("CODETETHER_POD_NAME").ok());

        let deployment_name = std::env::var("CODETETHER_DEPLOYMENT_NAME").ok();

        let client = super::bootstrap::client(&namespace, pod_name.as_deref()).await;

        Self {
            client,
            namespace,
            pod_name,
            deployment_name,
            actions: Arc::new(RwLock::new(Vec::new())),
            coordinator: coordination::MutationCoordinator,
        }
    }
}
