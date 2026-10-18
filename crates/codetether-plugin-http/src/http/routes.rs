//! Plugin discovery HTTP route assembly.
use super::{HttpState, handler};
use axum::{Router, routing::get};

/// Build the plugin discovery route using the caller's registry and identity.
/// # Returns
/// The plugin-list route ready to merge beneath the server's middleware.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_plugin_http::http::HttpState>() {
/// let routes = codetether_plugin_http::http::router::<S>();
/// # }
/// ```
pub fn router<S: HttpState>() -> Router<S> {
    Router::new().route("/v1/plugins", get(handler::list_plugins::<S>))
}
