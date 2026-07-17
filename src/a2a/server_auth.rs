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

#[path = "server_auth_token.rs"]
mod token;
pub use token::constant_time_eq;

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
    let Some(configured_token) = cached_token() else {
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
    if !token::accepted(provided, configured_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(request).await)
}
