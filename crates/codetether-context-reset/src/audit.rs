//! Audit helper for `context_reset`.

use serde_json::json;

/// Record an accepted context reset summary.
pub async fn log(bytes: usize) {
    tracing::info!(target: "codetether_agent::tool::context_reset::audit",bytes, "context_reset: agent-authored summary accepted");
    if let Some(audit) = codetether_audit::try_audit_log() {
        audit
            .log(
                codetether_audit::AuditCategory::ToolExecution,
                "tool:context_reset",
                codetether_audit::AuditOutcome::Success,
                None,
                Some(json!({ "bytes": bytes })),
            )
            .await;
    }
}
