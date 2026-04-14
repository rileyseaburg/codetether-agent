use super::{bearer_token, query_token};
use axum::{body::Body, http::Request};

/// Extract a token from either the `Authorization` header or `token` query parameter.
///
/// # Examples
///
/// ```rust
/// use axum::{body::Body, http::Request};
/// use codetether_agent::server::auth::provided_token;
///
/// let request = Request::builder()
///     .uri("/stream?token=query-token")
///     .body(Body::empty())
///     .expect("request");
///
/// assert_eq!(provided_token(&request), Some("query-token"));
/// ```
pub fn provided_token(request: &Request<Body>) -> Option<&str> {
    bearer_token(request).or_else(|| query_token(request))
}
