//! Existing server error-message classification and response text.
use http::StatusCode;

/// Convert an application failure into the server's status and message pair.
/// # Arguments
/// * `error` - Failure returned by an application operation.
/// # Returns
/// Not-found for messages containing `not found`, bad-request for messages
/// containing `disabled`, `exceeds` or `limit`, and internal-error otherwise.
/// Matching is case-sensitive and preserves the original message text.
/// # Examples
/// ```
/// let (status, message) = codetether_http_error::internal_error(anyhow::anyhow!("item not found"));
/// assert_eq!(status, http::StatusCode::NOT_FOUND);
/// assert_eq!(message, "item not found");
/// ```
pub fn internal_error(error: anyhow::Error) -> (StatusCode, String) {
    let message = error.to_string();
    if message.contains("not found") {
        return (StatusCode::NOT_FOUND, message);
    }
    if message.contains("disabled") || message.contains("exceeds") || message.contains("limit") {
        return (StatusCode::BAD_REQUEST, message);
    }
    (StatusCode::INTERNAL_SERVER_ERROR, message)
}
