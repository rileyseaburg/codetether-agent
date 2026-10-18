//! Kubernetes management and K8s-backed worker status routes.
use super::{HttpState, commands, queries};
use axum::{
    Router,
    routing::{get, post},
};
/// Build the Kubernetes status and lifecycle route group.
/// # Returns
/// Typed routes beneath the caller's authentication and policy layers.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_k8s_api::http::HttpState>() {
/// let routes = codetether_k8s_api::http::router::<S>();
/// # }
/// ```
pub fn router<S: HttpState>() -> Router<S> {
    Router::new()
        .route("/v1/k8s/status", get(queries::get_k8s_status::<S>))
        .route("/v1/k8s/scale", post(commands::k8s_scale::<S>))
        .route("/v1/k8s/restart", post(commands::k8s_restart::<S>))
        .route("/v1/k8s/pods", get(queries::k8s_list_pods::<S>))
        .route("/v1/k8s/actions", get(queries::k8s_actions::<S>))
        .route("/v1/k8s/subagent", post(commands::k8s_spawn_subagent::<S>))
        .route(
            "/v1/k8s/subagent/{id}",
            axum::routing::delete(commands::k8s_delete_subagent::<S>),
        )
}
/// Build both compatibility routes for connected-worker status.
/// # Returns
/// The worker and agent-worker list routes over the same manager.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_k8s_api::http::HttpState>() {
/// let routes = codetether_k8s_api::http::worker_routes::<S>();
/// # }
/// ```
pub fn worker_routes<S: HttpState>() -> Router<S> {
    Router::new()
        .route(
            "/v1/worker/connected",
            get(queries::list_connected_workers::<S>),
        )
        .route(
            "/v1/agent/workers",
            get(queries::list_connected_workers::<S>),
        )
}
