//! Anthropic error response types.
//!
//! Defines the deserializable error payloads returned by Anthropic's Messages
//! API and Anthropic-compatible providers. These types mirror the API response
//! shape so provider code can extract both a human-readable error message and
//! the optional machine-readable error classification.

use serde::Deserialize;

/// Top-level error envelope returned by the Messages API.
///
/// Anthropic error responses wrap the detailed failure information inside an
/// `error` object. This structure represents that outer response body and is
/// intended to be deserialized directly from a failed HTTP response.
///
/// The envelope has no behavior of its own; callers inspect [`Self::error`] to
/// obtain the detailed provider message and optional error type.
#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicError {
    /// Detailed error object supplied by the Anthropic-compatible API.
    pub(crate) error: AnthropicErrorDetail,
}

/// Detailed error payload returned by Anthropic-compatible APIs.
///
/// Contains the provider's diagnostic message and, when supplied, the
/// machine-readable error category from the JSON `type` field. Compatible
/// providers may omit `type`, so the field defaults to `None` during
/// deserialization.
///
/// The `message` field is suitable for surfacing in logs or higher-level error
/// contexts, while `error_type` can be used for conditional handling such as
/// rate-limit, authentication, or invalid-request cases.
#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicErrorDetail {
    /// Human-readable explanation of the API failure returned by the provider.
    pub(crate) message: String,
    /// Optional machine-readable error category from the provider's JSON `type` field.
    #[serde(default, rename = "type")]
    pub(crate) error_type: Option<String>,
}
