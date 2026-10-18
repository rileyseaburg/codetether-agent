//! Cognition events HTTP handlers.
use super::{HttpState, Lifecycle};
use axum::response::sse::{Event, Sse};
use axum::{Json, extract::State, http::StatusCode};
use codetether_cognition_core::MemorySnapshot;
use std::convert::Infallible;

pub(super) async fn stream_cognition<S: HttpState>(
    State(state): State<S>,
) -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    crate::stream_cognition(state.runtime().subscribe_events())
}

pub(super) async fn get_latest_snapshot<S: HttpState>(
    State(state): State<S>,
) -> Result<Json<MemorySnapshot>, (StatusCode, String)> {
    crate::latest_snapshot(state.runtime().latest_snapshot().await)
}
