//! Bearer-token middleware for the A2A HTTP surface.
//!
//! Closes the LAN-injection hole on the peer router: when
//! `CODETETHER_AUTH_TOKEN` is set, every request except the public
//! agent-card paths must carry a matching `Authorization: Bearer`.
//!
//! The token is cached in a [`OnceLock`] to avoid the env-var global
//! lock on every request.
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

/// Paths that stay public even when auth is enabled.
const PUBLIC_PATHS: [&str; 2] = ["/.well-known/agent.json", "/.well-known/agent-card.json"];

/// Cached auth token. Read once from the environment to avoid the
/// per-request `env::var` global lock; subsequent calls hit the
/// process-local `OnceLock`.
static AUTH_TOKEN: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();

fn cached_token() -> Option<&'static str> {
    AUTH_TOKEN
        .get_or_init(|| {
            std::env::var("CODETETHER_AUTH_TOKEN")
                .ok()
                .filter(|s| !s.is_empty())
        })
        .as_deref()
}

/// Enforce bearer auth on A2A RPC calls when a token is configured.
pub async fn require_a2a_auth(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let Some(expected) = cached_token() else {
        return Ok(next.run(request).await);
    };
    if PUBLIC_PATHS.contains(&request.uri().path()) {
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

/// Length-padded constant-time byte comparison. Always walks the longer
/// of the two inputs so a length mismatch does not shortcut the
/// comparison and leak token length via timing.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let len_diff = (a.len() ^ b.len()) as u8;
    let mut diff = len_diff;
    for i in 0..a.len().max(b.len()) {
        let left = a.get(i).copied().unwrap_or(0);
        let right = b.get(i).copied().unwrap_or(0);
        diff |= left ^ right;
    }
    diff == 0
}
