//! OpenAI-compatible error conversion.
use axum::{Json, http::StatusCode};

pub type OpenAiApiError = (StatusCode, Json<serde_json::Value>);

pub type OpenAiApiResult<T> = Result<T, OpenAiApiError>;

/// Build an OpenAI-shaped API error envelope.
/// # Arguments
/// status, message, error_type and code define the HTTP error.
/// # Returns
/// The status paired with an error JSON body.
/// # Examples
/// ```
/// assert_eq!(codetether_openai_adapter::openai_error(axum::http::StatusCode::BAD_REQUEST, "bad", "type", "code").0.as_u16(), 400);
/// ```
pub fn openai_error(
    status: StatusCode,
    message: impl Into<String>,
    error_type: &str,
    code: &str,
) -> OpenAiApiError {
    (
        status,
        Json(serde_json::json!({
            "error": {
                "message": message.into(),
                "type": error_type,
                "code": code,
            }
        })),
    )
}

/// Build an invalid-request API error.
/// # Arguments
/// message describes the invalid request.
/// # Returns
/// HTTP 400 paired with the error JSON.
/// # Examples
/// ```
/// assert_eq!(codetether_openai_adapter::openai_bad_request("bad").0.as_u16(), 400);
/// ```
pub fn openai_bad_request(message: impl Into<String>) -> OpenAiApiError {
    openai_error(
        StatusCode::BAD_REQUEST,
        message,
        "invalid_request_error",
        "invalid_request",
    )
}

/// Build an internal-server API error.
/// # Arguments
/// message describes the server failure.
/// # Returns
/// HTTP 500 paired with the error JSON.
/// # Examples
/// ```
/// assert_eq!(codetether_openai_adapter::openai_internal_error("failed").0.as_u16(), 500);
/// ```
pub fn openai_internal_error(message: impl Into<String>) -> OpenAiApiError {
    openai_error(
        StatusCode::INTERNAL_SERVER_ERROR,
        message,
        "server_error",
        "internal_error",
    )
}
