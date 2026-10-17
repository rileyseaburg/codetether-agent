//! Create a pod, replacing one stale conflicting pod before retry.

use anyhow::{Context, Result};
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, api::PostParams};
pub(super) async fn create(
    pods: &Api<Pod>,
    create_params: &PostParams,
    pod_manifest: &Pod,
    pod_name: &str,
    subagent_id: &str,
) -> Result<()> {
    match pods.create(create_params, pod_manifest).await {
        Ok(_) => {}
        Err(kube::Error::Api(api_err)) if api_err.code == 409 => {
            tracing::warn!(
                pod = %pod_name,
                subagent_id = %subagent_id,
                "Sub-agent pod already exists, deleting stale pod and retrying create"
            );
            let _ = pods
                .delete(&pod_name, &kube::api::DeleteParams::default())
                .await;
            tokio::time::sleep(std::time::Duration::from_millis(600)).await;
            pods.create(create_params, pod_manifest)
                .await
                .with_context(|| {
                    format!("Failed to create sub-agent pod {} after retry", pod_name)
                })?;
        }
        Err(e) => {
            return Err(e).with_context(|| format!("Failed to create sub-agent pod {pod_name}"));
        }
    }

    Ok(())
}
