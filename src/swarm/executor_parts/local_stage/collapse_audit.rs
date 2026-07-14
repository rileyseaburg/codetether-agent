//! Audit records for local branch promotion and termination.

use crate::audit::{AuditCategory, AuditOutcome};

pub(super) async fn promotion(swarm: &str, subtask: &str) {
    let Some(audit) = crate::audit::try_audit_log() else {
        return;
    };
    audit
        .log_with_correlation(
            AuditCategory::Swarm,
            "collapse_promote_branch",
            AuditOutcome::Success,
            Some("collapse-controller".into()),
            Some(serde_json::json!({ "swarm_id": swarm, "subtask_id": subtask })),
            None,
            None,
            Some(swarm.into()),
            None,
        )
        .await;
}

pub(super) async fn kill(swarm: &str, subtask: &str, branch: &str, reason: &str) {
    let Some(audit) = crate::audit::try_audit_log() else {
        return;
    };
    audit
        .log_with_correlation(
            AuditCategory::Swarm,
            "collapse_kill_branch",
            AuditOutcome::Success,
            Some("collapse-controller".into()),
            Some(
                serde_json::json!({ "swarm_id": swarm, "subtask_id": subtask,
            "branch": branch, "reason": reason }),
            ),
            None,
            None,
            Some(swarm.into()),
            None,
        )
        .await;
}
