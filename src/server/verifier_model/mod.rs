//! Authenticated verifier configuration; outer server layers enforce auth and policy.

mod defaults;
mod error;
mod handlers;
mod input;
mod projection;
mod source;
mod state;
mod types;
use axum::{Extension, Router, routing::get};
use state::ApiState;

pub(super) fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    routes(ApiState::default())
}

fn routes<S: Clone + Send + Sync + 'static>(state: ApiState) -> Router<S> {
    Router::new()
        .route(
            "/api/config/verifier-model",
            get(handlers::read)
                .put(handlers::replace)
                .delete(handlers::clear),
        )
        .layer(Extension(state))
}

#[cfg(test)]
mod tests;
