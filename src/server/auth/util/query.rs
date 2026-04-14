use axum::{body::Body, http::Request};

/// Extract a token from the `token` query parameter.
///
/// # Examples
///
/// ```rust
/// use axum::{body::Body, http::Request};
/// use codetether_agent::server::auth::query_token;
///
/// let request = Request::builder()
///     .uri("/stream?token=query-token")
///     .body(Body::empty())
///     .expect("request");
///
/// assert_eq!(query_token(&request), Some("query-token"));
/// ```
pub fn query_token(request: &Request<Body>) -> Option<&str> {
    request.uri().query().and_then(|query| {
        query
            .split('&')
            .find_map(|pair| match pair.split_once('=') {
                Some(("token", value)) => Some(value),
                _ => None,
            })
    })
}
