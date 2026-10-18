//! Voice session and voice catalog route assembly.
use super::{HttpState, handlers};
use crate::{get_voice_session_rest, list_voices_rest};
use axum::{
    Router,
    routing::{get, post},
};

/// Build the voice session and catalog routes using the caller's state.
/// # Returns
/// Routes ready to merge beneath the server's authentication and audit layers.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_voice_api::http::HttpState>() {
/// let routes = codetether_voice_api::http::router::<S>();
/// # }
/// ```
pub fn router<S: HttpState>() -> Router<S> {
    Router::new()
        .route(
            "/v1/voice/sessions",
            post(handlers::create_voice_session_rest::<S>),
        )
        .route(
            "/v1/voice/sessions/{room_name}",
            get(get_voice_session_rest).delete(handlers::delete_voice_session_rest::<S>),
        )
        .route("/v1/voice/voices", get(list_voices_rest))
}
