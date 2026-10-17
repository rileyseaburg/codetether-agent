//! Select in-cluster configuration before local kubeconfig fallback.

use kube::{Client, Config};
pub(super) async fn client(namespace: &str, pod_name: Option<&str>) -> Option<Client> {
    match Config::incluster() {
        Ok(config) => super::in_cluster::client(config, namespace, pod_name),
        Err(_) => super::kubeconfig::client(namespace).await,
    }
}
