//! CORS policy for the main HTTP listener.
use ::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;
/// Build the credentialed HTTP origin, method and header allowlist.
/// # Returns
/// The current explicit production/local browser access policy.
/// # Panics
/// Panics if a compiled-in origin stops being a valid header value.
/// # Examples
/// ```
/// let cors = codetether_server_cors::http();
/// ```
pub fn http() -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            "https://codetether.run".parse::<HeaderValue>().unwrap(),
            "https://api.codetether.run".parse::<HeaderValue>().unwrap(),
            "https://docs.codetether.run"
                .parse::<HeaderValue>()
                .unwrap(),
            "https://codetether.com".parse::<HeaderValue>().unwrap(),
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:3001".parse::<HeaderValue>().unwrap(),
            "http://localhost:8000".parse::<HeaderValue>().unwrap(),
        ])
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            ::http::header::AUTHORIZATION,
            ::http::header::CONTENT_TYPE,
            ::http::header::ACCEPT,
            ::http::header::ORIGIN,
            ::http::header::CACHE_CONTROL,
            ::http::header::HeaderName::from_static("x-worker-id"),
            ::http::header::HeaderName::from_static("x-agent-name"),
            ::http::header::HeaderName::from_static("x-workspaces"),
            ::http::header::HeaderName::from_static("x-requested-with"),
        ])
}
