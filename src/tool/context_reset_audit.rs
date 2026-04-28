//! Audit helper for `context_reset`.

use serde_json::json;

/// Record an accepted context reset summary.
pub async fn log(bytes: usize) {
    tracing::info!(bytes, "context_reset: agent-authored summary accepted");
    if let Some(audit) = crate::audit::try_audit_log() {
        audit
            .log(
                crate::audit::AuditCategory::ToolExecution,
                "tool:context_reset",
                crate::audit::AuditOutcome::Success,
                None,
                Some(json!({ "bytes": bytes })),
            )
            .await;
    }
}
