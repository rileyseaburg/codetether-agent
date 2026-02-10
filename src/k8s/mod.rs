//! Kubernetes self-deployment and pod management.
//!
//! Allows the CodeTether agent to manage its own lifecycle on Kubernetes:
//! - Detect whether it is running inside a K8s cluster
//! - Read its own pod/deployment metadata
//! - Scale its own deployment replica count
//! - Perform rolling restarts
//! - Create new pods for swarm sub-agents
//! - Monitor pod health and recover from failures
//!
//! All K8s operations are audit-logged.

use anyhow::{Context, Result, anyhow};
use chrono::Utc;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    Api, Client, Config as KubeConfig,
    api::{ListParams, Patch, PatchParams, PostParams},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Status of the K8s integration.
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

/// Result of a self-deployment action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployAction {
    pub action: String,
    pub success: bool,
    pub message: String,
    pub timestamp: String,
}

/// Kubernetes self-deployment manager.
#[derive(Clone)]
pub struct K8sManager {
    client: Option<Client>,
    namespace: String,
    pod_name: Option<String>,
    deployment_name: Option<String>,
    actions: Arc<RwLock<Vec<DeployAction>>>,
}

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

impl K8sManager {
    /// Attempt to initialize from in-cluster configuration.
    /// Returns a manager even if not running in K8s (with client = None).
    pub async fn new() -> Self {
        let namespace = std::env::var("CODETETHER_K8S_NAMESPACE")
            .or_else(|_| Self::read_namespace_file())
            .unwrap_or_else(|_| "default".to_string());

        let pod_name = std::env::var("HOSTNAME")
            .ok()
            .or_else(|| std::env::var("CODETETHER_POD_NAME").ok());

        let deployment_name = std::env::var("CODETETHER_DEPLOYMENT_NAME").ok();

        let client = match KubeConfig::incluster() {
            Ok(config) => match Client::try_from(config) {
                Ok(c) => {
                    tracing::info!(
                        namespace = %namespace,
                        pod = pod_name.as_deref().unwrap_or("-"),
                        "K8s client initialized (in-cluster)"
                    );
                    Some(c)
                }
                Err(e) => {
                    tracing::debug!("Failed to create in-cluster K8s client: {}", e);
                    None
                }
            },
            Err(_) => {
                // Try loading from KUBECONFIG for local development.
                match KubeConfig::from_kubeconfig(&kube::config::KubeConfigOptions::default()).await
                {
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
                        tracing::debug!(
                            "Not running in K8s and no kubeconfig found — K8s features disabled"
                        );
                        None
                    }
                }
            }
        };

        Self {
            client,
            namespace,
            pod_name,
            deployment_name,
            actions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Read /var/run/secrets/kubernetes.io/serviceaccount/namespace
    fn read_namespace_file() -> Result<String, std::env::VarError> {
        std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/namespace")
            .map(|s| s.trim().to_string())
            .map_err(|_| std::env::VarError::NotPresent)
    }

    /// Whether K8s integration is available.
    pub fn is_available(&self) -> bool {
        self.client.is_some()
    }

    /// Get current status.
    pub async fn status(&self) -> K8sStatus {
        let (replicas, available) = if let Some(ref client) = self.client {
            if let Some(ref dep_name) = self.deployment_name {
                self.get_deployment_replicas(client, dep_name).await
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        K8sStatus {
            in_cluster: self.client.is_some(),
            namespace: self.namespace.clone(),
            pod_name: self.pod_name.clone(),
            deployment_name: self.deployment_name.clone(),
            replicas,
            available_replicas: available,
        }
    }

    async fn get_deployment_replicas(
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

    /// Scale the agent's own deployment.
    pub async fn scale(&self, replicas: i32) -> Result<DeployAction> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available — cannot scale"))?;
        let dep_name = self
            .deployment_name
            .as_ref()
            .ok_or_else(|| anyhow!("Deployment name not set — set CODETETHER_DEPLOYMENT_NAME"))?;

        let deployments: Api<Deployment> = Api::namespaced(client.clone(), &self.namespace);

        let patch = serde_json::json!({
            "spec": {
                "replicas": replicas
            }
        });

        deployments
            .patch(
                dep_name,
                &PatchParams::apply("codetether"),
                &Patch::Merge(&patch),
            )
            .await
            .with_context(|| {
                format!(
                    "Failed to scale deployment {} to {} replicas",
                    dep_name, replicas
                )
            })?;

        let action = DeployAction {
            action: format!("scale:{}", replicas),
            success: true,
            message: format!("Scaled deployment '{}' to {} replicas", dep_name, replicas),
            timestamp: Utc::now().to_rfc3339(),
        };

        tracing::info!(
            deployment = %dep_name,
            replicas = replicas,
            "Self-deployment: scaled"
        );

        self.record_action(action.clone()).await;
        Ok(action)
    }

    /// Perform a rolling restart of the agent's deployment.
    pub async fn rolling_restart(&self) -> Result<DeployAction> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available — cannot restart"))?;
        let dep_name = self
            .deployment_name
            .as_ref()
            .ok_or_else(|| anyhow!("Deployment name not set — set CODETETHER_DEPLOYMENT_NAME"))?;

        let deployments: Api<Deployment> = Api::namespaced(client.clone(), &self.namespace);

        // Trigger rolling restart by updating the restart annotation.
        let patch = serde_json::json!({
            "spec": {
                "template": {
                    "metadata": {
                        "annotations": {
                            "codetether.run/restartedAt": Utc::now().to_rfc3339()
                        }
                    }
                }
            }
        });

        deployments
            .patch(
                dep_name,
                &PatchParams::apply("codetether"),
                &Patch::Merge(&patch),
            )
            .await
            .with_context(|| format!("Failed to trigger rolling restart for {}", dep_name))?;

        let action = DeployAction {
            action: "rolling_restart".to_string(),
            success: true,
            message: format!("Triggered rolling restart for deployment '{}'", dep_name),
            timestamp: Utc::now().to_rfc3339(),
        };

        tracing::info!(deployment = %dep_name, "Self-deployment: rolling restart");

        self.record_action(action.clone()).await;
        Ok(action)
    }

    /// List pods belonging to our deployment.
    pub async fn list_pods(&self) -> Result<Vec<PodInfo>> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available"))?;

        let pods: Api<Pod> = Api::namespaced(client.clone(), &self.namespace);

        let label_selector = self
            .deployment_name
            .as_ref()
            .map(|n| format!("app={}", n))
            .unwrap_or_else(|| "app=codetether".to_string());

        let list = pods
            .list(&ListParams::default().labels(&label_selector))
            .await
            .context("Failed to list pods")?;

        let infos: Vec<PodInfo> = list
            .items
            .iter()
            .map(|pod| {
                let name = pod.metadata.name.clone().unwrap_or_default();
                let phase = pod
                    .status
                    .as_ref()
                    .and_then(|s| s.phase.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                let ready = pod
                    .status
                    .as_ref()
                    .and_then(|s| s.conditions.as_ref())
                    .map(|conditions| {
                        conditions
                            .iter()
                            .any(|c| c.type_ == "Ready" && c.status == "True")
                    })
                    .unwrap_or(false);
                let start_time = pod
                    .status
                    .as_ref()
                    .and_then(|s| s.start_time.as_ref())
                    .map(|t| t.0.to_rfc3339());

                PodInfo {
                    name,
                    phase,
                    ready,
                    start_time,
                }
            })
            .collect();

        Ok(infos)
    }

    /// Spawn a new pod for a swarm sub-agent.
    pub async fn spawn_subagent_pod(
        &self,
        subagent_id: &str,
        image: Option<&str>,
        env_vars: HashMap<String, String>,
    ) -> Result<DeployAction> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available — cannot spawn sub-agent pod"))?;

        let pods: Api<Pod> = Api::namespaced(client.clone(), &self.namespace);

        let image = image.unwrap_or("ghcr.io/rileyseaburg/codetether-agent:latest");
        let pod_name = format!(
            "codetether-subagent-{}",
            &subagent_id[..8.min(subagent_id.len())]
        );

        let mut env_list: Vec<serde_json::Value> = env_vars
            .iter()
            .map(|(k, v)| serde_json::json!({ "name": k, "value": v }))
            .collect();
        env_list
            .push(serde_json::json!({ "name": "CODETETHER_SUBAGENT_ID", "value": subagent_id }));
        env_list.push(
            serde_json::json!({ "name": "CODETETHER_K8S_NAMESPACE", "value": &self.namespace }),
        );

        let pod_manifest: Pod = serde_json::from_value(serde_json::json!({
            "apiVersion": "v1",
            "kind": "Pod",
            "metadata": {
                "name": pod_name,
                "namespace": &self.namespace,
                "labels": {
                    "app": "codetether",
                    "codetether.run/role": "subagent",
                    "codetether.run/subagent-id": subagent_id
                }
            },
            "spec": {
                "restartPolicy": "Never",
                "containers": [{
                    "name": "agent",
                    "image": image,
                    "env": env_list,
                    "resources": {
                        "requests": { "memory": "256Mi", "cpu": "250m" },
                        "limits": { "memory": "1Gi", "cpu": "1000m" }
                    }
                }]
            }
        }))?;

        pods.create(&PostParams::default(), &pod_manifest)
            .await
            .with_context(|| format!("Failed to create sub-agent pod {}", pod_name))?;

        let action = DeployAction {
            action: format!("spawn_subagent:{}", subagent_id),
            success: true,
            message: format!(
                "Created sub-agent pod '{}' in namespace '{}'",
                pod_name, self.namespace
            ),
            timestamp: Utc::now().to_rfc3339(),
        };

        tracing::info!(
            pod = %pod_name,
            subagent_id = %subagent_id,
            "Self-deployment: spawned sub-agent pod"
        );

        self.record_action(action.clone()).await;
        Ok(action)
    }

    /// Delete a sub-agent pod.
    pub async fn delete_subagent_pod(&self, subagent_id: &str) -> Result<DeployAction> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available"))?;

        let pods: Api<Pod> = Api::namespaced(client.clone(), &self.namespace);
        let pod_name = format!(
            "codetether-subagent-{}",
            &subagent_id[..8.min(subagent_id.len())]
        );

        pods.delete(&pod_name, &kube::api::DeleteParams::default())
            .await
            .with_context(|| format!("Failed to delete pod {}", pod_name))?;

        let action = DeployAction {
            action: format!("delete_subagent:{}", subagent_id),
            success: true,
            message: format!("Deleted sub-agent pod '{}'", pod_name),
            timestamp: Utc::now().to_rfc3339(),
        };

        self.record_action(action.clone()).await;
        Ok(action)
    }

    /// Get recent deployment actions.
    pub async fn recent_actions(&self, limit: usize) -> Vec<DeployAction> {
        let actions = self.actions.read().await;
        actions.iter().rev().take(limit).cloned().collect()
    }

    async fn record_action(&self, action: DeployAction) {
        let mut actions = self.actions.write().await;
        actions.push(action);
        // Keep bounded.
        while actions.len() > 1000 {
            actions.remove(0);
        }
    }
}

/// Summary information about a pod.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodInfo {
    pub name: String,
    pub phase: String,
    pub ready: bool,
    pub start_time: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn k8s_manager_initializes_without_cluster() {
        let mgr = K8sManager::new().await;
        // In CI/local dev, likely no K8s cluster available.
        let status = mgr.status().await;
        assert_eq!(status.namespace.is_empty(), false);
    }
}
