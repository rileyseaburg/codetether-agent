//! Redacted HTTP errors with the relay's exact `{"error": ...}` shape.
use crate::reply::json;
use axum::response::{IntoResponse, Response};

/// HTTP status plus a redacted message; never carries credentials or pixels.
///
/// ```
/// use codetether_companion_relay::ApiError;
/// let error = ApiError::new(409, "Session changed while uploading");
/// assert_eq!(error.status, 409);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    /// HTTP status code.
    pub status: u16,
    /// Client-visible message.
    pub message: String,
}
impl ApiError {
    /// Build an error from a status and redacted message.
    pub fn new(status: u16, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }
    /// The relay's generic unmatched-route error.
    pub fn not_found() -> Self {
        Self::new(404, "Route not found")
    }
}
impl From<codetether_companion_core::Error> for ApiError {
    fn from(error: codetether_companion_core::Error) -> Self {
        Self {
            status: error.status(),
            message: error.to_string(),
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        json(self.status, &serde_json::json!({ "error": self.message }))
    }
}
