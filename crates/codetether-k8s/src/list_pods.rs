//! Pod listing for [`K8sManager`].
//!
//! Selector precedence: explicit `label_selector` → deployment scope
//! (`app=<deployment>`) → none. With no selector and no deployment scope,
//! **all** pods in the namespace are listed (no hidden filter). API errors
//! (including RBAC `Forbidden`) propagate rather than collapsing to empty.

use super::list_pods_selector::resolve_selector;
use super::{K8sManager, PodInfo};
use anyhow::{Context, Result, anyhow};
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, api::ListParams};

impl K8sManager {
    /// List pods in the manager's namespace using the default selector rules.
    pub async fn list_pods(&self) -> Result<Vec<PodInfo>> {
        self.list_pods_with_selector(None).await
    }

    /// List pods using an optional label selector override. See module docs
    /// for selector precedence and error semantics.
    pub async fn list_pods_with_selector(&self, sel: Option<&str>) -> Result<Vec<PodInfo>> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("K8s client not available"))?;
        let pods: Api<Pod> = Api::namespaced(client.clone(), &self.namespace);
        let selector = resolve_selector(sel, self.deployment_name.as_deref());
        let mut params = ListParams::default();
        if let Some(s) = selector.as_deref() {
            params = params.labels(s);
        }
        let ns = &self.namespace;
        let list = pods
            .list(&params)
            .await
            .with_context(|| format!("Failed to list pods in '{ns}' (selector {selector:?})"))?;
        Ok(list.items.iter().map(pod_info).collect())
    }
}

/// Project a [`Pod`] into the summarized [`PodInfo`].
fn pod_info(pod: &Pod) -> PodInfo {
    let s = pod.status.as_ref();
    PodInfo {
        name: pod.metadata.name.clone().unwrap_or_default(),
        phase: s
            .and_then(|s| s.phase.clone())
            .unwrap_or_else(|| "Unknown".into()),
        ready: s
            .and_then(|s| s.conditions.as_ref())
            .map(|c| c.iter().any(|c| c.type_ == "Ready" && c.status == "True"))
            .unwrap_or(false),
        start_time: s
            .and_then(|s| s.start_time.as_ref())
            .map(|t| t.0.to_string()),
    }
}
