//! HTTP state extraction for recording and querying audit history.
use super::HttpState;
use crate::AuditQuery;
use axum::{
    Json,
    body::Body,
    extract::{Query, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

/// Record a request using the audit log supplied by application state.
/// # Arguments
/// * `state` - Application state containing the shared audit log.
/// * `request` - Incoming HTTP request.
/// * `next` - Remaining middleware and request handler.
/// # Returns
/// The original response after recording the request's outcome and duration.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_audit_http::http::HttpState>(state: S, request: axum::http::Request<axum::body::Body>, next: axum::middleware::Next) {
/// let response = codetether_audit_http::http::audit_request(
///     axum::extract::State(state), request, next,
/// ).await;
/// # }
/// ```
pub async fn audit_request<S: HttpState>(
    State(state): State<S>,
    request: Request<Body>,
    next: Next,
) -> Response {
    crate::audit_middleware(state.audit_log(), request, next).await
}

pub(super) async fn list_audit_entries<S: HttpState>(
    State(state): State<S>,
    Query(query): Query<AuditQuery>,
) -> Result<Json<Vec<codetether_audit::AuditEntry>>, (StatusCode, String)> {
    crate::list_audit_entries(state.audit_log(), query).await
}
