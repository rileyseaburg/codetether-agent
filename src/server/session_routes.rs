//! Focused HTTP routes for persisted session management.

#[path = "session_routes/create.rs"]
mod create;
#[path = "session_routes/get.rs"]
mod get;
#[path = "session_routes/list.rs"]
mod list;
#[path = "session_routes/prompt.rs"]
mod prompt;

/// Build the non-realtime session HTTP surface.
pub(super) fn router() -> axum::Router<super::AppState> {
    use axum::routing::{get, post};

    axum::Router::new()
        .route("/api/session", get(list::list).post(create::create))
        .route("/api/session/{id}", get(get::get))
        .route("/api/session/{id}/prompt", post(prompt::prompt))
}
