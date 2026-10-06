//! Lease-gated Deployment rolling restarts.

use super::{DeployAction, K8sManager};
use anyhow::{Context, Result, anyhow};
use chrono::Utc;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{
    Api,
    api::{Patch, PatchParams},
};

impl K8sManager {
    pub async fn rolling_restart(&self) -> Result<DeployAction> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available — cannot restart"))?;
        let name = self
            .deployment_name
            .as_ref()
            .ok_or_else(|| anyhow!("Deployment name not set — set CODETETHER_DEPLOYMENT_NAME"))?;
        let claim = self
            .coordinator
            .acquire(
                client.clone(),
                &self.namespace,
                "Deployment",
                name,
                "rolling-restart",
            )
            .await?;
        let api: Api<Deployment> = Api::namespaced(client.clone(), &self.namespace);
        let mut annotations = claim.annotations();
        annotations.insert("codetether.run/restartedAt".into(), Utc::now().to_rfc3339());
        let patch = super::deployment_patch::restart(annotations);
        api.patch(
            name,
            &PatchParams::apply(claim.field_manager()),
            &Patch::Merge(&patch),
        )
        .await
        .with_context(|| format!("Failed to trigger rolling restart for {name}"))?;
        let action = DeployAction {
            action: "rolling_restart".into(),
            success: true,
            message: format!("Triggered rolling restart for deployment '{name}'"),
            timestamp: Utc::now().to_rfc3339(),
        };
        self.record_action(action.clone()).await;
        claim.release().await;
        Ok(action)
    }
}
