//! Delete a pod while treating a missing pod as already deleted.

use anyhow::{Context, Result};
use k8s_openapi::api::core::v1::Pod;
use kube::Api;
pub(super) async fn delete(pods: &Api<Pod>, pod_name: &str, subagent_id: &str) -> Result<()> {
    match pods
        .delete(&pod_name, &kube::api::DeleteParams::default())
        .await
    {
        Ok(_) => {}
        Err(kube::Error::Api(api_err)) if api_err.code == 404 => {
            tracing::debug!(
                pod = %pod_name,
                subagent_id = %subagent_id,
                "Sub-agent pod already deleted"
            );
        }
        Err(e) => {
            return Err(e).with_context(|| format!("Failed to delete pod {}", pod_name));
        }
    }

    Ok(())
}
