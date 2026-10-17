//! Lease-gated sub-agent pod launch orchestration.

use crate::{DeployAction, K8sManager, SubagentPodSpec};
use anyhow::{Result, anyhow};
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, api::PostParams};
impl K8sManager {
    /// Spawn a new pod for a swarm sub-agent with full pod options.
    ///
    /// # Errors
    /// Returns an error if the client is unavailable, mutation ownership cannot be acquired,
    /// or the required Kubernetes API operation fails.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_k8s::{K8sManager, SubagentPodSpec};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = K8sManager::new().await;
    /// let action = manager.spawn_subagent_pod_with_spec("task", SubagentPodSpec::default()).await?;
    /// assert!(action.success);
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn spawn_subagent_pod_with_spec(
        &self,
        subagent_id: &str,
        spec: SubagentPodSpec,
    ) -> Result<DeployAction> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available — cannot spawn sub-agent pod"))?;

        let pod_name = Self::subagent_pod_name(subagent_id);
        let claim = self
            .coordinator
            .acquire(
                client.clone(),
                &self.namespace,
                "Pod",
                &pod_name,
                "spawn-subagent",
            )
            .await?;
        let pods: Api<Pod> = Api::namespaced(client.clone(), &self.namespace);

        let pod_manifest = super::build::manifest(
            &pod_name,
            &self.namespace,
            subagent_id,
            &spec,
            claim.annotations(),
        )?;
        let create_params = PostParams {
            field_manager: Some(claim.field_manager().into()),
            ..PostParams::default()
        };
        super::create::create(&pods, &create_params, &pod_manifest, &pod_name, subagent_id).await?;
        let action = super::actions::spawned(subagent_id, &pod_name, &self.namespace);

        self.record_action(action.clone()).await;
        claim.release().await;
        Ok(action)
    }
}
