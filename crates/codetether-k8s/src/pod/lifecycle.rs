//! Extract the first container termination, reason, and restart count.

use k8s_openapi::api::core::v1::Pod;
pub(super) fn state(pod: &Pod) -> (bool, Option<i32>, Option<String>, u32) {
    let container_status = pod
        .status
        .as_ref()
        .and_then(|s| s.container_statuses.as_ref())
        .and_then(|statuses| statuses.first());
    let terminated = container_status
        .and_then(|status| status.state.as_ref())
        .and_then(|state| state.terminated.as_ref())
        .is_some();
    let exit_code = container_status
        .and_then(|status| status.state.as_ref())
        .and_then(|state| state.terminated.as_ref())
        .map(|terminated| terminated.exit_code);
    let reason = container_status
        .and_then(|status| status.state.as_ref())
        .and_then(|state| {
            state
                .terminated
                .as_ref()
                .and_then(|t| t.reason.clone())
                .or_else(|| state.waiting.as_ref().and_then(|w| w.reason.clone()))
        });
    let restart_count = container_status
        .map(|status| status.restart_count.max(0) as u32)
        .unwrap_or(0);

    (terminated, exit_code, reason, restart_count)
}
