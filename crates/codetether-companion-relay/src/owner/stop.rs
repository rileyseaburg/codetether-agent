//! End one live owner session and return the terminal response.
use crate::{ApiError, Shared, relay::now, reply::json};
use axum::response::Response;
use codetether_companion_protocol::Stopped;
pub(super) fn stop(relay: &Shared, id: &str) -> Result<Response, ApiError> {
    let time = now();
    let mut state = relay.lock();
    state.live(id, time)?;
    state.end(id, time);
    Ok(json(200, &Stopped { stopped: true }))
}
