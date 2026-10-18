//! Build authorization identities from authenticated HTTP request context.
//!
//! from_request maps decoded claims or static-token defaults into PolicyUser.
//! Static-token admin opt-in is read once, preserving process-local policy.
use crate::PolicyUser;
use axum::{body::Body, http::Request};
use codetether_http_auth::JwtClaims;
mod claims;
mod static_identity;
use claims::from_claims;
use static_identity::{static_token_admin_enabled, static_user};

/// Project authenticated request claims or static-token policy into user context.
/// # Arguments
/// * `request` - Request after authentication middleware.
/// # Returns
/// User context used by local or OPA authorization.
/// # Examples
/// ```no_run
/// let request = axum::http::Request::new(axum::body::Body::empty());
/// let _user = codetether_authorization_policy::user::from_request(&request);
/// ```
pub fn from_request(request: &Request<Body>) -> PolicyUser {
    if let Some(claims) = request.extensions().get::<JwtClaims>() {
        return from_claims(claims);
    }
    if static_token_admin_enabled() {
        return static_user(vec!["admin".to_string()], "static_token_admin");
    }
    static_user(Vec::new(), "static_token")
}
#[cfg(test)]
mod tests;
