//! Usage-accounting conversion for Anthropic responses.
//!
//! This module maps token accounting from Anthropic-compatible response
//! payloads into the provider-neutral [`Usage`] structure. It preserves prompt,
//! completion, total, and prompt-cache token counts when the provider includes
//! them, while defaulting missing aggregate usage to zero.

use crate::provider::Usage;

use super::response::AnthropicUsage;

/// Convert optional Anthropic usage metadata into provider-neutral token usage.
///
/// Anthropic-compatible providers may omit the `usage` object from some
/// responses. When usage is absent, prompt, completion, and total token counts
/// are reported as zero while cache-specific counts remain `None`. When usage
/// is present, total tokens are computed as `input_tokens + output_tokens`.
///
/// # Parameters
///
/// * `usage` - Optional reference to the usage object parsed from an
///   Anthropic-compatible response.
///
/// # Returns
///
/// A [`Usage`] value containing prompt, completion, total, cache-read, and
/// cache-write token counts in the application's common format.
///
/// # Side Effects
///
/// This function performs no I/O and does not mutate the supplied usage data.
pub(crate) fn usage(usage: Option<&AnthropicUsage>) -> Usage {
    Usage {
        prompt_tokens: usage.map(|u| u.input_tokens).unwrap_or(0),
        completion_tokens: usage.map(|u| u.output_tokens).unwrap_or(0),
        total_tokens: usage.map(|u| u.input_tokens + u.output_tokens).unwrap_or(0),
        cache_read_tokens: usage.and_then(|u| u.cache_read_input_tokens),
        cache_write_tokens: usage.and_then(|u| u.cache_creation_input_tokens),
    }
}
