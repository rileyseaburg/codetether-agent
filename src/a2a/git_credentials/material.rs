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

/// Short-lived Git credential material issued by the server.
///
/// The host and path fields are optional because Git may already have supplied
/// them in the original credential-helper request.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(creds.token_type, "github_app");
/// ```
#[derive(Debug, serde::Deserialize)]
pub struct GitCredentialMaterial {
    pub username: String,
    pub password: String,
    pub expires_at: Option<String>,
    pub token_type: String,
    pub host: Option<String>,
    pub path: Option<String>,
}
