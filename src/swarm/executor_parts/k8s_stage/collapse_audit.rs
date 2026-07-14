//! Audit records for Kubernetes branch collapse.

use crate::audit::{AuditCategory, AuditOutcome};

async fn write(action: &str, swarm: &str, metadata: serde_json::Value) {
    let Some(audit) = crate::audit::try_audit_log() else {
        return;
    };
    audit
        .log_with_correlation(
            AuditCategory::Swarm,
            action,
            AuditOutcome::Success,
            Some("collapse-controller".into()),
            Some(metadata),
            None,
            None,
            Some(swarm.into()),
            None,
        )
        .await;
}

pub(super) async fn promotion(swarm: &str, id: &str) {
    write(
        "collapse_promote_branch",
        swarm,
        serde_json::json!({
            "swarm_id": swarm, "subtask_id": id, "execution_mode": "kubernetes_pod",
        }),
    )
    .await;
}

pub(super) async fn kill(swarm: &str, id: &str, branch: &str, reason: &str) {
    write(
        "collapse_kill_branch",
        swarm,
        serde_json::json!({
            "swarm_id": swarm, "subtask_id": id, "branch": branch, "reason": reason,
            "execution_mode": "kubernetes_pod",
        }),
    )
    .await;
}
