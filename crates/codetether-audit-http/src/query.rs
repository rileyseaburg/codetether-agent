//! Audit history filtering and bounded API responses.
use crate::AuditQuery;
use axum::{Json, http::StatusCode};
use codetether_audit::{self as audit, AuditCategory, AuditLog};

/// Query recent audit records, optionally filtered by category.
/// # Arguments
/// * `log` - Shared audit history.
/// * `query` - Limit and category; omitted limits default to 100 and cap at 1000.
/// # Returns
/// The selected records as JSON.
/// # Errors
/// Returns HTTP bad request for an unrecognized category.
/// # Examples
/// ```no_run
/// # async fn example(log: &codetether_audit::AuditLog) {
/// let query = codetether_audit_http::AuditQuery { limit: None, category: None };
/// let _ = codetether_audit_http::list_audit_entries(log, query).await;
/// # }
/// ```
pub async fn list_audit_entries(
    log: &AuditLog,
    query: AuditQuery,
) -> Result<Json<Vec<audit::AuditEntry>>, (StatusCode, String)> {
    let limit = query.limit.unwrap_or(100).min(1000);

    let entries = if let Some(ref cat) = query.category {
        let category = match cat.as_str() {
            "api" => AuditCategory::Api,
            "tool" | "tool_execution" => AuditCategory::ToolExecution,
            "session" => AuditCategory::Session,
            "cognition" => AuditCategory::Cognition,
            "swarm" => AuditCategory::Swarm,
            "auth" => AuditCategory::Auth,
            "k8s" => AuditCategory::K8s,
            "sandbox" => AuditCategory::Sandbox,
            "config" => AuditCategory::Config,
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Unknown category: {}", cat),
                ));
            }
        };
        log.by_category(category, limit).await
    } else {
        log.recent(limit).await
    };

    Ok(Json(entries))
}
