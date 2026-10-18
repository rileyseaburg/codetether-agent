//! Configuration, trust and application version HTTP routes.
//!
//! Callers supply the loaded configuration and application package identity.
//! Binary hashing is performed per request, and verifier settings share the
//! existing process-wide verifier stores.
mod config;
mod health;
mod metadata;
mod metadata_routes;
mod state;
mod version;
use codetether_config::Config;
pub use metadata_routes::{discovery_routes, health_routes};
use std::sync::Arc;
/// Build status routes under caller-owned authentication and policy layers.
/// # Arguments
/// * `config` - Loaded application configuration.
/// * `version` - Application package version, supplied by the executable.
/// * `name` - Application package name, supplied by the executable.
/// # Returns
/// Configuration, trust status, version and verifier settings routes.
/// # Examples
/// ```no_run
/// # fn example(config: std::sync::Arc<codetether_config::Config>) {
/// let _router = codetether_server_status::router::<()>(config, "1.0", "application");
/// # }
/// ```
pub fn router<S: Clone + Send + Sync + 'static>(
    config: Arc<Config>,
    version: &'static str,
    name: &'static str,
) -> axum::Router<S> {
    use axum::{Extension, routing::get};
    axum::Router::new()
        .route("/api/version", get(version::get_version))
        .route("/api/config", get(config::get_config))
        .route("/api/config/trust-status", get(config::get_trust_status))
        .merge(codetether_verifier_http::router())
        .layer(Extension(state::StatusState {
            config,
            version,
            name,
        }))
}
