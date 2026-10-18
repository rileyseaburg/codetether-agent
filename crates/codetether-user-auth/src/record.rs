//! Shared user authentication record operations.

use serde::Deserialize;

/// Stored credentials from the user login command.
///
/// # Examples
/// ```
/// let record: codetether_user_auth::SavedCredentials = serde_json::from_value(serde_json::json!({
///     "server": "https://example.invalid", "access_token": "example", "expires_at": "unknown"
/// })).unwrap();
/// assert_eq!(record.email, "");
/// ```
#[derive(Debug, Deserialize)]
pub struct SavedCredentials {
    /// Server base URL.
    pub server: String,
    /// Cached access token.
    pub access_token: String,
    /// Server-supplied expiry string.
    pub expires_at: String,
    #[serde(default)]
    /// Account email, defaulting to empty for older records.
    pub email: String,
}
