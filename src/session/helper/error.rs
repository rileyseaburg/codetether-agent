//! Backward-compatible provider error helper exports.

pub use super::error_context::messages_to_rlm_context;
use super::error_detect::{is_prompt_too_long_message, is_retryable_upstream_message};

/// Returns true when an upstream provider rejected the prompt as too large.
pub fn is_prompt_too_long_error(err: &anyhow::Error) -> bool {
    is_prompt_too_long_message(&err.to_string())
}

/// Returns true when an upstream provider error can be retried or failed over.
pub fn is_retryable_upstream_error(err: &anyhow::Error) -> bool {
    is_retryable_upstream_message(&err.to_string())
}
