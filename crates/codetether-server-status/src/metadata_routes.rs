//! Health and compatibility metadata route groups.
use crate::{health, metadata};
use axum::{Router, routing::get};

/// Build the basic process-health route.
/// # Returns
/// A health route whose authentication exemption remains caller policy.
/// # Examples
/// ```
/// let routes = codetether_server_status::health_routes::<()>();
/// ```
pub fn health_routes<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route("/health", get(health::health))
}

/// Build provider-name and built-in-agent discovery routes.
/// # Returns
/// Compatibility metadata routes to merge under the server's middleware.
/// # Examples
/// ```
/// let routes = codetether_server_status::discovery_routes::<()>();
/// ```
pub fn discovery_routes<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/api/provider", get(metadata::list_providers))
        .route("/api/agent", get(metadata::list_agents))
}
