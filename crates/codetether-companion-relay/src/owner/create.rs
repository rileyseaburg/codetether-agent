//! Create a session from the owner's validated input.
use crate::{ApiError, Shared, body, relay::now, reply::json};
use axum::{body::Body, http::HeaderMap, response::Response};
use codetether_companion_protocol::SessionInput;
/// `POST /companion/sessions`.
pub(crate) async fn create(
    relay: &Shared,
    headers: &HeaderMap,
    raw: Body,
) -> Result<Response, ApiError> {
    let value = body::read_json(headers, raw, body::SMALL).await?;
    let input: SessionInput =
        serde_json::from_value(value).map_err(|_| codetether_companion_core::Error::Input)?;
    Ok(json(200, &relay.lock().create(input, now())?))
}
