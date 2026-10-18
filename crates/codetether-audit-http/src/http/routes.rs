//! Audit history route assembly.
use super::{HttpState, handlers};
use axum::{Router, routing::get};

/// Build the audit history route using the caller's application state.
/// # Returns
/// The history route ready for the server's authentication and policy layers.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_audit_http::http::HttpState>() {
/// let routes = codetether_audit_http::http::router::<S>();
/// # }
/// ```
pub fn router<S: HttpState>() -> Router<S> {
    Router::new().route("/v1/audit", get(handlers::list_audit_entries::<S>))
}
