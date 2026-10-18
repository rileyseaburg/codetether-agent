//! Verify scope using the caller's token, never worker credentials or JWT claims.
use super::scope::ScopeError;
use axum::http::HeaderMap;
use std::path::PathBuf;
mod request;
mod response;

/// Ask the configured authority for an authorized workspace binding.
/// # Errors
/// Fails closed on missing credentials, rejection or authority failures.
/// # Arguments
/// * `server` - Authority base URL.
/// * `id` - Requested codebase identity.
/// * `headers` - Caller request headers containing the Bearer token.
/// # Returns
/// Absolute workspace path from the matching authority response.
/// # Examples
/// ```no_run
/// # async fn example(headers: &axum::http::HeaderMap) {
/// let _ = codetether_session_access::authority::workspace("https://authority.example", "codebase", headers).await;
/// # }
/// ```
pub async fn workspace(server: &str, id: &str, headers: &HeaderMap) -> anyhow::Result<PathBuf> {
    let response = request::build(server, id, headers)?
        .send()
        .await
        .map_err(|_| ScopeError::Unavailable)?;
    response::binding(response, id).await
}
