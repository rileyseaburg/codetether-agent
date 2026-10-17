//! Kubernetes manager actions.

use crate::DeployAction;
use crate::K8sManager;
impl K8sManager {
    /// Get recent deployment actions.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_k8s::{K8sManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = K8sManager::new().await;
    /// assert!(manager.recent_actions(10).await.len() <= 10);
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn recent_actions(&self, limit: usize) -> Vec<DeployAction> {
        let actions = self.actions.read().await;
        actions.iter().rev().take(limit).cloned().collect()
    }

    pub(crate) async fn record_action(&self, action: DeployAction) {
        let mut actions = self.actions.write().await;
        actions.push(action);
        // Keep bounded.
        while actions.len() > 1000 {
            actions.remove(0);
        }
    }
}
