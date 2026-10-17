//! Context-window estimates use the shared provider policy.
//! Context-window and max-output-token estimates for Bedrock models.
//!
//! *Context-window* estimates delegate to the canonical
//! [`codetether_provider_core::limits::context_window_for_model`] so a single map
//! covers all providers.  *Max-output* estimates are Bedrock-specific
//! (they vary by inference-profile, not by model family) and live here.
//!
//! # Examples
//!
//! ```rust
//! use codetether_bedrock_protocol::{estimate_context_window, estimate_max_output};
//!
//! assert_eq!(estimate_context_window("us.anthropic.claude-opus-4-7"), 1_000_000);
//! assert_eq!(estimate_max_output("us.anthropic.claude-opus-4-7"), 128_000);
//! assert_eq!(estimate_context_window("amazon.nova-lite-v1:0"), 300_000);
//! ```

/// Estimate context-window size (input+output combined) for a Bedrock model.
///
/// Delegates to [`codetether_provider_core::limits::context_window_for_model`].
///
/// # Arguments
///
/// * `model_id` — Full or aliased Bedrock model identifier.
///
/// # Returns
///
/// Estimated token capacity; returns 128 000 for unknown families.
///
/// # Examples
///
/// ```rust
/// use codetether_bedrock_protocol::estimate_context_window;
/// assert_eq!(estimate_context_window("anthropic.claude-3-haiku-20240307-v1:0"), 200_000);
/// assert_eq!(estimate_context_window("unknown.model"), 128_000);
/// ```
pub fn estimate_context_window(model_id: &str) -> usize {
    codetether_provider_core::limits::context_window_for_model(model_id)
}
