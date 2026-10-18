//! Shared user authentication reply operations.

use serde::Deserialize;

/// Successful server login response used to persist user credentials.
///
/// # Examples
/// ```
/// let reply = codetether_user_auth::LoginResponsePayload {
///     access_token: "example".into(), expires_at: "2030-01-01T00:00:00Z".into(),
///     user: serde_json::json!({}),
/// };
/// assert!(reply.user.is_object());
/// ```
#[derive(Debug, Deserialize)]
pub struct LoginResponsePayload {
    /// Access token returned by the server.
    pub access_token: String,
    /// Server-supplied expiry string.
    pub expires_at: String,
    /// Additional server-supplied user metadata.
    pub user: serde_json::Value,
}
