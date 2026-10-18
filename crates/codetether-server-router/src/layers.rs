//! Mandatory authentication, authorization, auditing and HTTP transport layers.
use crate::HttpState;
use axum::{Router, middleware};
use codetether_authorization_policy::policy_middleware;
use codetether_http_auth::{AuthState, require_auth};
use tower_http::trace::TraceLayer;

pub(super) fn apply<S: HttpState>(app: Router, state: S, auth: AuthState) -> Router {
    app.layer(middleware::from_fn_with_state(
        state.clone(),
        codetether_audit_http::http::audit_request::<S>,
    ))
    .layer(middleware::from_fn(policy_middleware))
    .layer(middleware::from_fn(require_auth))
    .layer(axum::Extension(auth))
    .layer(codetether_server_cors::http())
    .layer(TraceLayer::new_for_http())
}
