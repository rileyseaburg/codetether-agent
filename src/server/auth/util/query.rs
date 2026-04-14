use axum::{body::Body, http::Request};
use std::borrow::Cow;

/// Extract a token from the `token` query parameter.
///
/// # Examples
///
/// ```rust
/// use axum::{body::Body, http::Request};
/// use codetether_agent::server::auth::query_token;
///
/// let request = Request::builder()
///     .uri("/stream?token=query%20token")
///     .body(Body::empty())
///     .expect("request");
///
/// assert_eq!(query_token(&request).as_deref(), Some("query token"));
/// ```
pub fn query_token(request: &Request<Body>) -> Option<Cow<'_, str>> {
    request.uri().query().and_then(|query| {
        query.split('&').find_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            let decoded_key = urlencoding::decode(key).ok()?;
            if decoded_key != "token" {
                return None;
            }
            urlencoding::decode(value).ok()
        })
    })
}
