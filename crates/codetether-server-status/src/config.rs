//! Loaded configuration and effective trust status responses.
use crate::state::StatusState;
use axum::{Extension, Json};
use codetether_config::{Config, TrustPolicyStatus};

/// Return the loaded server configuration.
pub(super) async fn get_config(Extension(state): Extension<StatusState>) -> Json<Config> {
    Json((*state.config).clone())
}

/// Return effective trust, approval, sandbox, and permission-profile status.
pub(super) async fn get_trust_status(
    Extension(state): Extension<StatusState>,
) -> Json<TrustPolicyStatus> {
    Json(TrustPolicyStatus::from_config(&state.config))
}
