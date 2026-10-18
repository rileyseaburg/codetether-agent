//! Cognition and persona HTTP route assembly.
use super::{HttpState, persona_routes, runtime_routes, state_routes};
use axum::Router;
/// Build cognition lifecycle, persona and state-query routes.
/// # Returns
/// Routes ready to merge beneath the application's authentication and audit layers.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_cognition_api::http::HttpState>() {
/// let routes = codetether_cognition_api::http::router::<S>();
/// # }
/// ```
pub fn router<S: HttpState>() -> Router<S> {
    Router::new()
        .merge(runtime_routes::router::<S>())
        .merge(persona_routes::router::<S>())
        .merge(state_routes::router::<S>())
}
