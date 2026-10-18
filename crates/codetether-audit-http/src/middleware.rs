//! Request latency, outcome classification and audit recording.
use axum::{body::Body, http::Request, middleware::Next, response::Response};
use codetether_audit::{self as audit, AuditCategory, AuditLog, AuditOutcome};

/// Audit middleware — logs every request/response to the audit trail.
/// # Arguments
/// * `log` - Shared audit log to record into.
/// * `request` - Incoming HTTP request.
/// * `next` - Remaining middleware and handler chain.
/// # Returns
/// The original response after recording its status and elapsed time.
/// # Examples
/// ```no_run
/// # async fn example(log: &codetether_audit::AuditLog, request: axum::http::Request<axum::body::Body>, next: axum::middleware::Next) {
/// let _ = codetether_audit_http::audit_middleware(log, request, next).await;
/// # }
/// ```
pub async fn audit_middleware(log: &AuditLog, request: Request<Body>, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let started = std::time::Instant::now();

    let response = next.run(request).await;

    let duration_ms = started.elapsed().as_millis() as u64;
    let status = response.status().as_u16();
    let outcome = if status < 400 {
        AuditOutcome::Success
    } else if status == 401 || status == 403 {
        AuditOutcome::Denied
    } else {
        AuditOutcome::Failure
    };

    log.record(audit::AuditEntry {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now(),
        category: AuditCategory::Api,
        action: format!("{} {}", method, path),
        principal: None,
        outcome,
        detail: Some(serde_json::json!({ "status": status })),
        duration_ms: Some(duration_ms),
        okr_id: None,
        okr_run_id: None,
        relay_id: None,
        session_id: None,
    })
    .await;

    response
}
