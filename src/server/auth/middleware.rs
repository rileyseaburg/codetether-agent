use super::util::{constant_time_eq, provided_token};
use super::{AuthState, extract_jwt_claims};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

/// Axum middleware layer that enforces Bearer token auth on every request
/// except public paths.
///
/// # Examples
///
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use axum::{
///     Router,
///     body::Body,
///     http::{Request, StatusCode},
///     middleware,
///     routing::get,
/// };
/// use codetether_agent::server::auth::{AuthState, require_auth};
/// use tower::ServiceExt;
///
/// let app = Router::new()
///     .route("/secure", get(|| async { "ok" }))
///     .layer(middleware::from_fn(require_auth))
///     .layer(axum::Extension(AuthState::with_token("example-token")));
///
/// let response = app
///     .oneshot(
///         Request::builder()
///             .uri("/secure")
///             .header("authorization", "Bearer example-token")
///             .body(Body::empty())
///             .expect("request"),
///     )
///     .await
///     .expect("response");
///
/// assert_eq!(response.status(), StatusCode::OK);
/// # });
/// ```
pub async fn require_auth(mut request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let path = request.uri().path();
    let auth_state = request
        .extensions()
        .get::<AuthState>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    if auth_state.is_public_path(path) {
        return Ok(next.run(request).await);
    }
    let provided_token = provided_token(&request).ok_or(StatusCode::UNAUTHORIZED)?;
    if !constant_time_eq(provided_token.as_bytes(), auth_state.token().as_bytes()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if let Some(claims) = extract_jwt_claims(provided_token) {
        request.extensions_mut().insert(claims);
    }
    Ok(next.run(request).await)
}
