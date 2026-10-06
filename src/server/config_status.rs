//! Configuration status handlers.

use super::AppState;
use crate::config::{Config, TrustPolicyStatus};
use axum::{Json, extract::State};
#[path = "verifier_model/mod.rs"]
mod verifier_model;

/// Return the loaded server configuration.
pub(super) async fn get_config(State(state): State<AppState>) -> Json<Config> {
    Json((*state.config).clone())
}

/// Group configuration and version routes under the server's existing auth layers.
pub(super) fn router() -> axum::Router<AppState> {
    use axum::routing::get;
    axum::Router::new()
        .route("/api/version", get(super::version_info::get_version))
        .route("/api/config", get(get_config))
        .route("/api/config/trust-status", get(get_trust_status))
        .merge(verifier_model::router())
}

/// Return effective trust, approval, sandbox, and permission-profile status.
pub(super) async fn get_trust_status(State(state): State<AppState>) -> Json<TrustPolicyStatus> {
    Json(TrustPolicyStatus::from_config(&state.config))
}
