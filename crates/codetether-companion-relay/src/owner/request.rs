//! Validate the owner request body before mutating a live session.
use crate::{ApiError, Shared, body, relay::now, reply::json};
use axum::{body::Body, http::HeaderMap, response::Response};
pub(super) async fn request(
    relay: &Shared,
    id: &str,
    headers: &HeaderMap,
    raw: Body,
) -> Result<Response, ApiError> {
    relay.lock().live(id, now())?;
    let value = body::read_json(headers, raw, body::SMALL).await?;
    let time = now();
    let mut state = relay.lock();
    let paired = state.paired(id, time);
    Ok(json(
        202,
        &state.live(id, time)?.request(&value, paired, time)?,
    ))
}
