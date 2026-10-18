//! CORS policy for the gRPC-Web listener.
use ::http::HeaderValue;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
/// Build the gRPC-Web origin and exposed-status-header policy.
/// # Returns
/// The existing production/local origin list with any methods and headers.
/// # Examples
/// ```
/// let cors = codetether_server_cors::grpc();
/// ```
pub fn grpc() -> CorsLayer {
    let allowed_origins = [
        HeaderValue::from_static("https://codetether.run"),
        HeaderValue::from_static("https://api.codetether.run"),
        HeaderValue::from_static("https://docs.codetether.run"),
        HeaderValue::from_static("https://codetether.com"),
        HeaderValue::from_static("http://localhost:3000"),
        HeaderValue::from_static("http://localhost:3001"),
    ];
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .expose_headers(tower_http::cors::ExposeHeaders::list([
            ::http::header::HeaderName::from_static("grpc-status"),
            ::http::header::HeaderName::from_static("grpc-message"),
        ]))
}
