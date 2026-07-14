//! Best-effort deletion of Kubernetes sub-agent pods.

use super::state::State;

pub(super) async fn delete(state: &State<'_>, id: &str, reason: &str) {
    if let Err(error) = state.k8s.delete_subagent_pod(id).await {
        tracing::warn!(subtask_id = %id, %error, reason, "Failed deleting Kubernetes pod");
    }
}
