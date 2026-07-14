//! Lease-gated Deployment scaling.

use super::{DeployAction, K8sManager};
use anyhow::{Context, Result, anyhow};
use chrono::Utc;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{
    Api,
    api::{Patch, PatchParams},
};

impl K8sManager {
    pub async fn scale(&self, replicas: i32) -> Result<DeployAction> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available — cannot scale"))?;
        let name = self
            .deployment_name
            .as_ref()
            .ok_or_else(|| anyhow!("Deployment name not set — set CODETETHER_DEPLOYMENT_NAME"))?;
        let claim = self
            .coordinator
            .acquire(client.clone(), &self.namespace, "Deployment", name, "scale")
            .await?;
        let api: Api<Deployment> = Api::namespaced(client.clone(), &self.namespace);
        let patch = super::deployment_patch::scale(replicas, claim.annotations());
        api.patch(
            name,
            &PatchParams::apply(claim.field_manager()),
            &Patch::Merge(&patch),
        )
        .await
        .with_context(|| format!("Failed to scale deployment {name} to {replicas} replicas"))?;
        let action = DeployAction {
            action: format!("scale:{replicas}"),
            success: true,
            message: format!("Scaled deployment '{name}' to {replicas} replicas"),
            timestamp: Utc::now().to_rfc3339(),
        };
        tracing::info!(deployment = %name, replicas, "Self-deployment: scaled");
        self.record_action(action.clone()).await;
        claim.release().await;
        Ok(action)
    }
}
