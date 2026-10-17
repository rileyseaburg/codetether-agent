//! Kubernetes client, scope, action log, and mutation coordination state.

use crate::{DeployAction, coordination};
use kube::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Kubernetes self-deployment manager.
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
#[derive(Clone)]
pub struct K8sManager {
    pub(crate) client: Option<Client>,
    pub(crate) namespace: String,
    pub(crate) pod_name: Option<String>,
    pub(crate) deployment_name: Option<String>,
    pub(crate) actions: Arc<RwLock<Vec<DeployAction>>>,
    pub(crate) coordinator: coordination::MutationCoordinator,
}
