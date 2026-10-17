//! Exchange a one-use pairing code for the device credential.
use crate::{ApiError, Shared, body, relay::now, reply::json};
use axum::{body::Body, http::HeaderMap, response::Response};
use codetether_companion_protocol::PairRequest;
/// `POST /companion/pair`: exchange a one-use code for a device token.
pub(crate) async fn pair(
    relay: &Shared,
    headers: &HeaderMap,
    raw: Body,
) -> Result<Response, ApiError> {
    let value = body::read_json(headers, raw, body::SMALL).await?;
    let request: PairRequest =
        serde_json::from_value(value).map_err(|_| ApiError::new(400, "Pairing code required"))?;
    Ok(json(200, &relay.lock().pair(&request.code, now())?))
}
