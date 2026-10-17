//! Route owner session actions after the router checks owner credentials.
use crate::{ApiError, Shared, relay::now, sse};
use axum::{body::Body, http::HeaderMap, response::Response};
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
        ("POST", Some("request")) => super::request::request(relay, id, headers, raw).await,
        ("POST", Some("reply")) => super::reply::reply(relay, id, headers, raw).await,
        ("DELETE", None) => super::stop::stop(relay, id),
        _ => Err(ApiError::not_found()),
    }
}
