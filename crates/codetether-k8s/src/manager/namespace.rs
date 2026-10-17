//! Kubernetes manager namespace.

use crate::K8sManager;
use anyhow::Result;
impl K8sManager {
    /// Read /var/run/secrets/kubernetes.io/serviceaccount/namespace
    pub(super) fn read_namespace_file() -> Result<String, std::env::VarError> {
        std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/namespace")
            .map(|s| s.trim().to_string())
            .map_err(|_| std::env::VarError::NotPresent)
    }
}
