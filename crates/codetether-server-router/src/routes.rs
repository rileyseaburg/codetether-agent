//! Shared endpoint ordering, native extension insertion and A2A mounting.
use crate::{Extensions, HttpState, Transports, layers};
use axum::Router;
use codetether_http_auth::AuthState;
/// Compose the application router and install its mandatory middleware.
/// # Arguments
/// * `state` - Existing application state shared by all handlers.
/// * `extensions` - Native runtime adapter route groups.
/// * `transports` - Already-bound root and nested A2A routers.
/// * `auth` - Existing authentication state passed to the request layers.
/// # Returns
/// Fully bound router with shared routes and the existing middleware order.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_server_router::HttpState>(state: S, extensions: codetether_server_router::Extensions<S>, transports: codetether_server_router::Transports, auth: codetether_http_auth::AuthState) {
/// let app = codetether_server_router::build(state, extensions, transports, auth);
/// # }
/// ```
pub fn build<S: HttpState>(
    state: S,
    extensions: Extensions<S>,
    transports: Transports,
    auth: AuthState,
) -> Router {
    let app = extensions
        .approval
        .merge(codetether_server_status::health_routes::<S>())
        .merge(codetether_task_ingest::http::router::<S>())
        .merge(codetether_task_api::http::queue_routes::<S>())
        .merge(extensions.sessions)
        .merge(extensions.realtime)
        .merge(extensions.status)
        .merge(codetether_server_status::discovery_routes::<S>())
        .merge(extensions.models)
        .merge(extensions.completions)
        .merge(codetether_cognition_api::http::router::<S>())
        .merge(codetether_audit_http::http::router::<S>())
        .merge(codetether_event_replay::router::<S>())
        .merge(codetether_k8s_api::http::router::<S>())
        .merge(codetether_plugin_http::http::router::<S>())
        .merge(codetether_tool_discovery::router::<S>())
        .merge(codetether_task_api::http::agent_routes::<S>())
        .merge(extensions.workers)
        .merge(codetether_task_api::http::dispatch_routes::<S>())
        .merge(codetether_voice_api::http::router::<S>())
        .merge(extensions.resume)
        .merge(codetether_bus_sse::http::router::<S>())
        .with_state(state.clone())
        .merge(transports.root)
        .nest("/a2a", transports.nested);
    layers::apply(app, state, auth)
}
