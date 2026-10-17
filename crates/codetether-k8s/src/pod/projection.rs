//! Project a Kubernetes pod into its sub-agent state.

use crate::SubagentPodState;
use k8s_openapi::api::core::v1::Pod;
pub(super) fn state(pod_name: String, pod: &Pod) -> SubagentPodState {
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

    let (terminated, exit_code, reason, restart_count) = super::lifecycle::state(pod);
    SubagentPodState {
        pod_name,
        phase,
        ready,
        terminated,
        exit_code,
        reason,
        restart_count,
    }
}
