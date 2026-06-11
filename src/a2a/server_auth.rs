//! Bearer-token middleware for the A2A HTTP surface.
//!
//! Closes the LAN-injection hole on the peer router: when
//! `CODETETHER_AUTH_TOKEN` is set, every request except the public
//! agent-card paths must carry a matching `Authorization: Bearer`.
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
const PUBLIC_PATHS: [&str; 2] = ["/.well-known/agent.json", "/.well-known/agent-card.json"];
pub async fn require_a2a_auth(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let Ok(expected) = std::env::var("CODETETHER_AUTH_TOKEN") else {
        return Ok(next.run(request).await);
    };
    if expected.is_empty() || PUBLIC_PATHS.contains(&request.uri().path()) {
        return Ok(next.run(request).await);
    }
    let provided = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if !constant_time_eq(provided.as_bytes(), expected.as_bytes()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(request).await)
}
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}
