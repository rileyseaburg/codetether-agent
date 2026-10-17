//! Construct a local kubeconfig client when in-cluster configuration is absent.

use kube::{Client, Config};
pub(super) async fn client(namespace: &str) -> Option<Client> {
    match Config::from_kubeconfig(&kube::config::KubeConfigOptions::default()).await {
        Ok(config) => match Client::try_from(config) {
            Ok(c) => {
                tracing::info!(namespace = %namespace, "K8s client initialized (kubeconfig)");
                Some(c)
            }
            Err(e) => {
                tracing::debug!("Failed to create K8s client from kubeconfig: {}", e);
                None
            }
        },
        Err(_) => {
            tracing::debug!("Not running in K8s and no kubeconfig found — K8s features disabled");
            None
        }
    }
}
