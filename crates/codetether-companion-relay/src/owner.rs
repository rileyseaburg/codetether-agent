//! Owner-only handlers: create, observe, request a fresh frame, stop.
use crate::{ApiError, Shared, body, relay::now, reply::json, sse};
use axum::body::Body;
use axum::http::HeaderMap;
use axum::response::Response;
use codetether_companion_protocol::{SessionInput, Stopped};

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
/// `GET|POST|DELETE /companion/sessions/{id}[/events|/request]`.
pub(crate) async fn session(
    relay: &Shared,
    method: &str,
    id: &str,
    action: Option<&str>,
    headers: &HeaderMap,
    raw: Body,
) -> Result<Response, ApiError> {
    match (method, action) {
        ("GET", Some("events")) => sse::subscribe(relay.lock().live(id, now())?),
        ("POST", Some("request")) => {
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
        ("POST", Some("reply")) => {
            relay.lock().live(id, now())?;
            let value = body::read_json(headers, raw, body::SMALL).await?;
            let time = now();
            let mut state = relay.lock();
            let paired = state.paired(id, time);
            Ok(json(
                202,
                &state.live(id, time)?.queue_reply(&value, paired, time)?,
            ))
        }
        ("DELETE", None) => {
            let time = now();
            let mut state = relay.lock();
            state.live(id, time)?;
            state.end(id, time);
            Ok(json(200, &Stopped { stopped: true }))
        }
        _ => Err(ApiError::not_found()),
    }
}
