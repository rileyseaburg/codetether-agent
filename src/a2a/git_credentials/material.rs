//! Credential material returned by the A2A control plane.
//!
//! This module defines the token payload the worker receives when it asks the
//! server for short-lived Git credentials.
//!
//! # Examples
//!
//! ```ignore
//! let creds = GitCredentialMaterial {
//!     username: "x-access-token".to_string(),
//!     password: "secret".to_string(),
//!     expires_at: None,
//!     token_type: "github_app".to_string(),
//!     host: None,
//!     path: None,
//! };
//! ```

use std::fmt;

/// Short-lived Git credential material issued by the server.
///
/// The host and path fields are optional because Git may already have supplied
/// them in the original credential-helper request.
///
/// The `password` field is redacted in [`Debug`](std::fmt::Debug) output to
/// prevent accidental credential leakage in logs.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(creds.token_type, "github_app");
/// ```
#[derive(serde::Deserialize)]
pub struct GitCredentialMaterial {
    pub username: String,
    pub password: String,
    pub expires_at: Option<String>,
    pub token_type: String,
    pub host: Option<String>,
    pub path: Option<String>,
}

impl fmt::Debug for GitCredentialMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GitCredentialMaterial")
            .field("username", &self.username)
            .field("password", &"[REDACTED]")
            .field("expires_at", &self.expires_at)
            .field("token_type", &self.token_type)
            .field("host", &self.host)
            .field("path", &self.path)
            .finish()
    }
}
