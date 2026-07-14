//! Conversion of a terminated Kubernetes pod into a subtask result.

use super::{
    pod_cleanup,
    state::{ActiveBranch, State},
};
use crate::k8s::SubagentPodState;
use crate::swarm::{SubTaskResult, k8s_result};

pub(super) async fn build(
    state: &mut State<'_>,
    id: &str,
    active: &ActiveBranch,
    pod: &SubagentPodState,
) -> SubTaskResult {
    let logs = state
        .k8s
        .subagent_logs(id, 10_000)
        .await
        .unwrap_or_default();
    let mut result = k8s_result::final_result(
        id,
        active.started_at.elapsed().as_millis() as u64,
        pod.exit_code,
        pod.reason.as_deref(),
        &logs,
    );
    if let Some(reason) = state.kill_reasons.get(id) {
        result.success = false;
        result.error = Some(format!("Cancelled by collapse controller: {reason}"));
        result.result.push_str(&format!(
            "\n\n--- Collapse Controller ---\nBranch terminated: {reason}",
        ));
    }
    state.active.remove(id);
    pod_cleanup::delete(state, id, "completion").await;
    result
}
