//! Device-token handlers scoped to one paired session.
use crate::{ApiError, Shared, analysis, body, frame, relay::now, reply::json};
use axum::body::Body;
use axum::http::HeaderMap;
use axum::response::Response;
use codetether_companion_protocol::{Accepted, PairRequest, Paused, Typed, TypedAck};

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
/// `GET commands`, `POST pause`, `POST typed`, `POST frames` for the paired device.
pub(crate) async fn session(
    relay: &Shared,
    action: &str,
    id: &str,
    auth: Option<&str>,
    headers: &HeaderMap,
    raw: Body,
) -> Result<Response, ApiError> {
    if action == "typed" {
        let value = body::read_json(headers, raw, body::SMALL).await?;
        let ack: TypedAck = serde_json::from_value(value)
            .map_err(|_| ApiError::new(400, "Reply acknowledgement required"))?;
        let mut state = relay.lock();
        let typed = state.device(id, auth, now())?.ack_reply(&ack);
        return Ok(json(200, &Typed { typed }));
    }
    let epoch = {
        let time = now();
        let mut state = relay.lock();
        let rt = state.device(id, auth, time)?;
        match action {
            "commands" => return Ok(json(200, &rt.command(time))),
            "pause" => {
                rt.pause();
                return Ok(json(200, &Paused { paused: true }));
            }
            _ => rt.seq,
        }
    };
    let value = body::read_json(headers, raw, body::FRAME).await?;
    let time = now();
    let (capture, captured) = frame::validate(value, time)?;
    let work = {
        let mut state = relay.lock();
        let rt = state.device(id, auth, time)?;
        if rt.seq != epoch {
            return Err(ApiError::new(409, "Session changed while uploading"));
        }
        rt.admit(&capture, captured, time)?
    };
    analysis::spawn(relay.clone(), id.to_string(), capture.image, work);
    Ok(json(202, &Accepted { accepted: true }))
}
