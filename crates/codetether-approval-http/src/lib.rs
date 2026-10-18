//! HTTP approval request and decision routes.

mod decision;
mod error;
mod kind;
mod request;
mod response;
mod types;

use axum::{Router, routing::post};

/// Construct approval routes for inclusion under the caller's auth layers.
/// # Returns
/// A router with request and decision POST endpoints.
/// # Examples
/// ```
/// let _routes = codetether_approval_http::router::<()>();
/// ```
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/v1/tools/approvals/request", post(request::handle))
        .route("/v1/tools/approvals/{id}/decision", post(decision::handle))
}

#[cfg(test)]
#[path = "session_decision_tests.rs"]
mod session_decision_tests;
#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_env;
