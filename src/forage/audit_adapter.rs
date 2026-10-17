//! Initialize the application audit sink and publish correlated forage events.
use super::*;
use crate::audit::{self, AuditCategory, AuditLog, AuditOutcome};
use uuid::Uuid;

pub(super) async fn ensure_audit_log_initialized() {
    if audit::try_audit_log().is_some() {
        return;
    }

    let default_sink = crate::config::Config::data_dir()
        .map(|base| base.join("audit"))
        .map(|audit_dir| {
            let _ = std::fs::create_dir_all(&audit_dir);
            audit_dir.join("forage_audit.jsonl")
        });
    let log = if std::env::var("CODETETHER_AUDIT_LOG_PATH").is_ok() {
        AuditLog::from_env()
    } else {
        AuditLog::new(10_000, default_sink)
    };
    let _ = audit::init_audit_log(log);
}

pub(super) async fn log_audit(
    category: AuditCategory,
    action: &str,
    outcome: AuditOutcome,
    detail: Option<serde_json::Value>,
    okr_id: Option<Uuid>,
    session_id: Option<String>,
) {
    if let Some(audit_log) = audit::try_audit_log() {
        audit_log
            .log_with_correlation(
                category,
                action,
                outcome,
                Some("forage-runtime".to_string()),
                detail,
                okr_id.map(|id| id.to_string()),
                None,
                None,
                session_id,
            )
            .await;
    }
}
