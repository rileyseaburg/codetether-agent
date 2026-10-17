//! Redacted manager debug representation.

use crate::K8sManager;

impl std::fmt::Debug for K8sManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("K8sManager")
            .field("namespace", &self.namespace)
            .field("pod_name", &self.pod_name)
            .field("deployment_name", &self.deployment_name)
            .field("connected", &self.client.is_some())
            .finish()
    }
}
