//! Collapse observation for one Kubernetes sub-agent.

use super::{
    super::{kubernetes_executor, resource_health},
    state::{ActiveBranch, State},
};
use crate::swarm::BranchObservation;

pub(super) async fn build(state: &State<'_>, id: &str, active: &ActiveBranch) -> BranchObservation {
    let pod = match state.k8s.get_subagent_pod_state(id).await {
        Ok(pod) => pod,
        Err(error) => {
            tracing::warn!(subtask_id = %id, %error, "Failed querying pod for collapse sample");
            None
        }
    };
    let (resource_health_score, infra_unhealthy_signals) = resource_health::compute(pod.as_ref());
    let logs = state.k8s.subagent_logs(id, 500).await.unwrap_or_default();
    let probe = kubernetes_executor::latest_probe_from_logs(&logs);
    let compile_ok = probe
        .as_ref()
        .map(|probe| probe.compile_ok)
        .unwrap_or_else(|| {
            pod.as_ref()
                .is_some_and(|pod| !pod.phase.eq_ignore_ascii_case("failed"))
        });
    let changed_files = probe
        .as_ref()
        .map(kubernetes_executor::probe_changed_files_set)
        .unwrap_or_default();
    let changed_lines = probe.map(|probe| probe.changed_lines).unwrap_or(0);
    BranchObservation {
        subtask_id: id.into(),
        branch: active.branch.clone(),
        compile_ok,
        changed_files,
        changed_lines,
        resource_health_score,
        infra_unhealthy_signals,
    }
}
