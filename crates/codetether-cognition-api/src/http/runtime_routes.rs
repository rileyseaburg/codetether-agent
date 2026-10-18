//! Cognition runtime routes assembly.
use super::{HttpState, control, events};
use axum::{
    Router,
    routing::{get, post},
};
pub(super) fn router<S: HttpState>() -> Router<S> {
    Router::new()
        // Perpetual cognition APIs
        .route("/v1/cognition/start", post(control::start_cognition::<S>))
        .route("/v1/cognition/stop", post(control::stop_cognition::<S>))
        .route(
            "/v1/cognition/status",
            get(control::get_cognition_status::<S>),
        )
        .route("/v1/cognition/stream", get(events::stream_cognition::<S>))
        .route(
            "/v1/cognition/snapshots/latest",
            get(events::get_latest_snapshot::<S>),
        )
}
