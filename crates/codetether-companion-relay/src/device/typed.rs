//! Decode and acknowledge one queued device reply.
use crate::{ApiError, Shared, body, relay::now, reply::json};
use axum::{body::Body, http::HeaderMap, response::Response};
use codetether_companion_protocol::{Typed, TypedAck};
pub(super) async fn acknowledge(
    relay: &Shared,
    id: &str,
    auth: Option<&str>,
    headers: &HeaderMap,
    raw: Body,
) -> Result<Response, ApiError> {
    let value = body::read_json(headers, raw, body::SMALL).await?;
    let ack: TypedAck = serde_json::from_value(value)
        .map_err(|_| ApiError::new(400, "Reply acknowledgement required"))?;
    let mut state = relay.lock();
    let typed = state.device(id, auth, now())?.ack_reply(&ack);
    Ok(json(200, &Typed { typed }))
}
