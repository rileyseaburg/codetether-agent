//! Configuration status handlers.

use super::AppState;
use crate::config::{Config, TrustPolicyStatus};
use axum::{Json, extract::State};

/// Return the loaded server configuration.
pub(super) async fn get_config(State(state): State<AppState>) -> Json<Config> {
    Json((*state.config).clone())
}

/// Return effective trust, approval, sandbox, and permission-profile status.
pub(super) async fn get_trust_status(State(state): State<AppState>) -> Json<TrustPolicyStatus> {
    Json(TrustPolicyStatus::from_config(&state.config))
}
