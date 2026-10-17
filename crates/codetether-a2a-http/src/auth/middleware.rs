//! Authorize peer HTTP calls while retaining public card discovery.
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

/// Paths that stay public even when auth is enabled.
const PUBLIC_PATHS: [&str; 2] = ["/.well-known/agent.json", "/.well-known/agent-card.json"];

/// Enforce bearer auth on A2A RPC calls when a token is configured.
///
/// # Arguments
/// * `request` - Incoming peer HTTP request.
/// * `next` - Remaining Axum middleware chain.
/// # Returns
/// The downstream response for an authorized request.
/// # Errors
/// Returns HTTP 401 for missing or invalid credentials when a token is configured.
/// # Examples
/// ```rust
/// use axum::{Router, middleware};
/// use codetether_a2a_http::auth::require_a2a_auth;
/// let router: Router = Router::new().layer(middleware::from_fn(require_a2a_auth));
/// ```
pub async fn require_a2a_auth(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let Some(configured_token) = super::cached::cached_token() else {
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
    if !super::token::accepted(provided, configured_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(request).await)
}
