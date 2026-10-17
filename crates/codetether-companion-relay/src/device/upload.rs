//! Validate and admit a frame after rechecking the session sequence.
use crate::{ApiError, Shared, body, relay::now, reply::json};
use crate::{analysis, frame};
use axum::{body::Body, http::HeaderMap, response::Response};
use codetether_companion_protocol::Accepted;
pub(super) async fn receive(
    relay: &Shared,
    id: &str,
    auth: Option<&str>,
    headers: &HeaderMap,
    raw: Body,
    epoch: u64,
) -> Result<Response, ApiError> {
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
