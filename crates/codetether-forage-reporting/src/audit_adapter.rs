//! Initialize the application audit sink and publish correlated forage events.
use codetether_audit as audit;
use codetether_audit::{AuditCategory, AuditLog, AuditOutcome};
use uuid::Uuid;

/// Initialize forage's audit sink unless a process-wide sink already exists.
/// # Returns
/// Uses the existing sink, the configured environment path, or the default file.
/// # Examples
/// ```no_run
/// # async fn setup() {
/// codetether_forage_reporting::ensure_audit_log_initialized().await;
/// # }
/// ```
pub async fn ensure_audit_log_initialized() {
    if audit::try_audit_log().is_some() {
        return;
    }

    let default_sink = codetether_config::Config::data_dir()
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

/// Append an audit event correlated with the forage runtime and optional IDs.
/// # Arguments
/// Event category, action, outcome and detail describe the event; objective and
/// session IDs connect it to their durable records.
/// # Returns
/// Completes after logging, or does nothing when no audit sink is installed.
/// # Examples
/// ```no_run
/// # async fn report() {
/// use codetether_audit::{AuditCategory, AuditOutcome};
/// codetether_forage_reporting::log_audit(
///     AuditCategory::Cognition, "forage.stop", AuditOutcome::Success,
///     None, None, None,
/// ).await;
/// # }
/// ```
pub async fn log_audit(
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
