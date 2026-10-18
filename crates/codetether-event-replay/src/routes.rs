//! Session event archive replay route assembly.
use crate::{list_session_event_files, replay_session_events};
use axum::{Router, routing::get};

/// Build the archive replay and file-index routes.
/// # Returns
/// Routes ready to merge beneath the application's authentication and audit layers.
/// # Examples
/// ```
/// let routes = codetether_event_replay::router::<()>();
/// ```
pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/v1/audit/replay", get(replay_session_events))
        .route("/v1/audit/replay/index", get(list_session_event_files))
}
