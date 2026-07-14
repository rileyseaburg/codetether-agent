//! Polling of one active Kubernetes sub-agent pod.

use super::{
    failure, final_result,
    state::{ActiveBranch, State},
    timeout,
};
use crate::swarm::SubTaskResult;

pub(super) async fn execute(
    state: &mut State<'_>,
    id: &str,
    active: &ActiveBranch,
) -> Option<SubTaskResult> {
    if let Some(result) = timeout::check(state, id, active).await {
        return Some(result);
    }
    let pod = match state.k8s.get_subagent_pod_state(id).await {
        Ok(Some(pod)) => pod,
        Ok(None) => {
            state.active.remove(id);
            return Some(failure::result(
                id,
                "Sub-agent pod disappeared".into(),
                active.started_at.elapsed().as_millis() as u64,
            ));
        }
        Err(error) => {
            tracing::warn!(subtask_id = %id, %error, "Failed querying Kubernetes pod state");
            return None;
        }
    };
    let phase = pod.phase.to_ascii_lowercase();
    if !pod.terminated && phase != "succeeded" && phase != "failed" {
        return None;
    }
    Some(final_result::build(state, id, active, &pod).await)
}
