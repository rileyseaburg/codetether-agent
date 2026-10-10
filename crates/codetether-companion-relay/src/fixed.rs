//! Non-session paths: `/companion` redirect, web shell, create, and pair.
use crate::{ApiError, Shared, assets, device, owner};
use axum::body::Body;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};

/// Handle a fixed path, returning the body back when the path is a session route.
pub(crate) async fn handle(
    relay: &Shared,
    method: &str,
    path: &str,
    auth: Option<&str>,
    headers: &HeaderMap,
    raw: Body,
) -> Result<Result<Response, Body>, ApiError> {
    if method == "GET" && path == "/companion" {
        let location = [(header::LOCATION, "/companion/")];
        return Ok(Ok(
            (StatusCode::PERMANENT_REDIRECT, location).into_response()
        ));
    }
    if method == "GET"
        && let Some(asset) = assets::serve(relay.assets.as_deref(), path).await?
    {
        return Ok(Ok(asset));
    }
    match (method, path) {
        ("POST", "/companion/sessions") => {
            relay.owner.authorize(auth)?;
            owner::create(relay, headers, raw).await.map(Ok)
        }
        ("POST", "/companion/pair") => device::pair(relay, headers, raw).await.map(Ok),
        _ => Ok(Err(raw)),
    }
}
