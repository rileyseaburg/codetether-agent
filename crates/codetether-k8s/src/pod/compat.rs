//! Kubernetes manager compat.

use crate::K8sManager;
use crate::{DeployAction, SubagentPodSpec};
use anyhow::Result;
use std::collections::HashMap;
impl K8sManager {
    /// List pods belonging to our deployment.
    ///
    /// # Errors
    /// Returns an error if the client is unavailable, mutation ownership cannot be acquired,
    /// or the required Kubernetes API operation fails.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_k8s::{K8sManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = K8sManager::new().await;
    /// let action = manager.spawn_subagent_pod("task", None, Default::default()).await?;
    /// assert!(action.success);
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    #[allow(dead_code)]
    /// Spawn a new pod for a swarm sub-agent.
    pub async fn spawn_subagent_pod(
        &self,
        subagent_id: &str,
        image: Option<&str>,
        env_vars: HashMap<String, String>,
    ) -> Result<DeployAction> {
        self.spawn_subagent_pod_with_spec(
            subagent_id,
            SubagentPodSpec {
                image: image.map(ToString::to_string),
                env_vars,
                ..SubagentPodSpec::default()
            },
        )
        .await
    }
}
