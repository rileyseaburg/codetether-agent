//! Companion-prefix routing; device capabilities never reach owner routes.
use crate::{ApiError, Shared, device, fixed, owner, paths};
use axum::Router;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use codetether_companion_core::require_origin;

/// Build the relay router; every path goes through one dispatcher.
pub fn router(relay: Shared) -> Router {
    Router::new()
        .route("/", any(dispatch))
        .route("/{*path}", any(dispatch))
        .with_state(relay)
}
async fn dispatch(State(relay): State<Shared>, request: Request) -> Response {
    handle(&relay, request)
        .await
        .unwrap_or_else(IntoResponse::into_response)
}
async fn handle(relay: &Shared, request: Request) -> Result<Response, ApiError> {
    let (parts, raw): (_, Body) = request.into_parts();
    let get = |name| {
        parts
            .headers
            .get(name)
            .and_then(|v: &header::HeaderValue| v.to_str().ok())
    };
    require_origin(get(header::ORIGIN), &relay.origin)?;
    let auth = get(header::AUTHORIZATION);
    let (method, path, headers) = (parts.method.as_str(), parts.uri.path(), &parts.headers);
    let raw = match fixed::handle(relay, method, path, auth, headers, raw).await? {
        Ok(response) => return Ok(response),
        Err(raw) => raw,
    };
    let (id, action) = paths::session(path).ok_or_else(ApiError::not_found)?;
    match (method, action) {
        ("POST", Some(a @ ("frames" | "pause" | "typed"))) | ("GET", Some(a @ "commands")) => {
            device::session(relay, a, id, auth, headers, raw).await
        }
        _ => {
            relay.owner.authorize(auth)?;
            owner::session(relay, method, id, action, headers, raw).await
        }
    }
}
