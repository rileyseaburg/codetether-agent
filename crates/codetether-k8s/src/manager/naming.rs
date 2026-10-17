//! Kubernetes manager naming.

use crate::K8sManager;
use crate::subagent_name;
impl K8sManager {
    ///
    /// # Examples
    /// ```
    /// use codetether_k8s::{K8sManager};
    /// let name = K8sManager::subagent_pod_name("Task/1");
    /// assert!(name.starts_with("codetether-subagent-"));
    /// ```
    pub fn subagent_pod_name(subagent_id: &str) -> String {
        subagent_name::pod_name(subagent_id)
    }
}
