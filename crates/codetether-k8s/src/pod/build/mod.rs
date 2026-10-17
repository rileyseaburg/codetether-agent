//! Build a Kubernetes Pod from launch options and lease annotations.

mod environment;
mod labels;
use crate::SubagentPodSpec;
use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use std::collections::BTreeMap;
pub(super) fn manifest(
    pod_name: &str,
    namespace: &str,
    subagent_id: &str,
    spec: &SubagentPodSpec,
    annotations: BTreeMap<String, String>,
) -> Result<Pod> {
    let image = spec
        .image
        .as_deref()
        .unwrap_or("ghcr.io/rileyseaburg/codetether-agent:latest");

    let env_list = environment::entries(spec, subagent_id, namespace);
    let labels = labels::labels(spec, subagent_id);
    Ok(serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "name": pod_name,
            "namespace": namespace,
            "labels": labels,
            "annotations": annotations
        },
        "spec": {
            "restartPolicy": "Never",
            "containers": [{
                "name": "agent",
                "image": image,
                "env": env_list,
                "command": spec.command,
                "args": spec.args,
                "resources": {
                    "requests": { "memory": "256Mi", "cpu": "250m" },
                    "limits": { "memory": "1Gi", "cpu": "1000m" }
                }
            }]
        }
    }))?)
}
