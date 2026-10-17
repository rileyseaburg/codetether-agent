//! Successful pod mutation descriptions.

use crate::DeployAction;
use chrono::Utc;
pub(super) fn spawned(subagent_id: &str, pod_name: &str, namespace: &str) -> DeployAction {
    let action = DeployAction {
        action: format!("spawn_subagent:{}", subagent_id),
        success: true,
        message: format!(
            "Created sub-agent pod '{}' in namespace '{}'",
            pod_name, namespace
        ),
        timestamp: Utc::now().to_rfc3339(),
    };
    tracing::info!(
        pod = %pod_name,
        subagent_id = %subagent_id,
        "Self-deployment: spawned sub-agent pod"
    );
    action
}
pub(super) fn deleted(subagent_id: &str, pod_name: &str) -> DeployAction {
    DeployAction {
        action: format!("delete_subagent:{}", subagent_id),
        success: true,
        message: format!("Deleted sub-agent pod '{}'", pod_name),
        timestamp: Utc::now().to_rfc3339(),
    }
}
