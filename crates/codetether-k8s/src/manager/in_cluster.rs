//! Construct an in-cluster client and report availability.

use kube::{Client, Config};
pub(super) fn client(config: Config, namespace: &str, pod_name: Option<&str>) -> Option<Client> {
    match Client::try_from(config) {
        Ok(c) => {
            tracing::info!(namespace = %namespace, pod = pod_name.as_deref().unwrap_or("-"), "K8s client initialized (in-cluster)");
            Some(c)
        }
        Err(e) => {
            tracing::debug!("Failed to create in-cluster K8s client: {}", e);
            None
        }
    }
}
