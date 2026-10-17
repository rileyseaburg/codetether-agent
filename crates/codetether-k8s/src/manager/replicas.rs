//! Kubernetes manager replicas.

use crate::K8sManager;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{Api, Client};
impl K8sManager {
    pub(super) async fn get_deployment_replicas(
        &self,
        client: &Client,
        name: &str,
    ) -> (Option<i32>, Option<i32>) {
        let deployments: Api<Deployment> = Api::namespaced(client.clone(), &self.namespace);
        match deployments.get(name).await {
            Ok(dep) => {
                let spec_replicas = dep.spec.as_ref().and_then(|s| s.replicas);
                let available = dep.status.as_ref().and_then(|s| s.available_replicas);
                (spec_replicas, available)
            }
            Err(e) => {
                tracing::warn!("Failed to get deployment {}: {}", name, e);
                (None, None)
            }
        }
    }
}
