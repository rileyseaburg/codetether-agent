use axum::{
    body::Body,
    http::{Request, header},
};

/// Extract a token from the `Authorization: Bearer ...` header.
///
/// # Examples
///
/// ```rust
/// use axum::{body::Body, http::Request};
/// use codetether_agent::server::auth::bearer_token;
///
/// let request = Request::builder()
///     .header("authorization", "Bearer example-token")
///     .body(Body::empty())
///     .expect("request");
///
/// assert_eq!(bearer_token(&request), Some("example-token"));
/// ```
pub fn bearer_token(request: &Request<Body>) -> Option<&str> {
    request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
}
