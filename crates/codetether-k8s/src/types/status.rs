//! K8sStatus data contract.

use serde::{Deserialize, Serialize};

/// Status of the K8s integration.
///
/// # Examples
/// ```
/// use codetether_k8s::{K8sStatus};
/// let status = K8sStatus { in_cluster: false, namespace: "default".into(),
///     pod_name: None, deployment_name: None, replicas: None, available_replicas: None };
/// assert!(!status.in_cluster);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K8sStatus {
    /// Whether we are running inside a K8s cluster.
    pub in_cluster: bool,
    /// Current namespace.
    pub namespace: String,
    /// This pod's name (if detectable).
    pub pod_name: Option<String>,
    /// Deployment name managing this pod (if detectable).
    pub deployment_name: Option<String>,
    /// Current replica count.
    pub replicas: Option<i32>,
    /// Available replica count.
    pub available_replicas: Option<i32>,
}
