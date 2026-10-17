//! Dispatch device commands and capture the pre-upload session sequence.
use crate::{ApiError, Shared, relay::now, reply::json};
use axum::{body::Body, http::HeaderMap, response::Response};
use codetether_companion_protocol::Paused;
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
        return super::typed::acknowledge(relay, id, auth, headers, raw).await;
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
    super::upload::receive(relay, id, auth, headers, raw, epoch).await
}
