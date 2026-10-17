//! Manager k8s_manager_initializes_without_cluster regression.

use super::*;

#[tokio::test]
async fn k8s_manager_initializes_without_cluster() {
    let mgr = K8sManager::new().await;
    // In CI/local dev, likely no K8s cluster available.
    let status = mgr.status().await;
    assert_eq!(status.namespace.is_empty(), false);
}
