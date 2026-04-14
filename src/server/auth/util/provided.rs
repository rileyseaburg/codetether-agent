use super::{bearer_token, query_token};
use axum::{body::Body, http::Request};
use std::borrow::Cow;

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
/// assert_eq!(provided_token(&request).as_deref(), Some("query-token"));
/// ```
pub fn provided_token(request: &Request<Body>) -> Option<Cow<'_, str>> {
    bearer_token(request)
        .map(Cow::Borrowed)
        .or_else(|| query_token(request))
}
