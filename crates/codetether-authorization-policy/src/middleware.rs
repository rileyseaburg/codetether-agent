//! Permission lookup and enforcement for authenticated HTTP requests.
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

/// Policy authorization middleware for Axum.
///
/// Maps request paths to OPA permission strings and enforces authorization.
/// Runs after `require_auth` so the bearer token is already validated.
/// # Arguments
/// * `request` - Request with authentication extensions already populated.
/// * `next` - Next middleware or handler.
/// # Returns
/// Downstream response when the permission is exempt or authorized.
/// # Errors
/// Returns forbidden for unmapped or denied requests.
/// # Examples
/// ```
/// let _router: axum::Router = axum::Router::new()
///     .route("/health", axum::routing::get(|| async { "ok" }))
///     .layer(axum::middleware::from_fn(codetether_authorization_policy::policy_middleware));
/// ```
pub async fn policy_middleware(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let path = request.uri().path();
    let method = request.method().as_str();

    let permission = match crate::routes::required_permission(path, method)? {
        None => return Ok(next.run(request).await),
        Some(permission) => permission,
    };

    let user = crate::user::from_request(&request);

    if let Err(status) = crate::enforce_policy(&user, permission, None).await {
        tracing::warn!(
            path = %path,
            method = %method,
            permission = %permission,
            "Policy middleware denied request"
        );
        return Err(status);
    }

    Ok(next.run(request).await)
}
