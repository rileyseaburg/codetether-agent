//! Short-name aliases for common Bedrock model IDs.
//!
//! Allows users to specify e.g. `claude-sonnet-4` instead of the full
//! `us.anthropic.claude-sonnet-4-20250514-v1:0`. Unknown strings are returned
//! unchanged so full model IDs pass through.
//!
//! # Examples
//!
//! ```rust
//! use codetether_provider_core::model_identity::bedrock::resolve_model_id;
//!
//! assert_eq!(
//!     resolve_model_id("claude-opus-4-6"),
//!     "us.anthropic.claude-opus-4-6-v1"
//! );
//! // Unknown IDs pass through unchanged
//! assert_eq!(resolve_model_id("custom.model-id"), "custom.model-id");
//! ```

#[path = "aliases_claude_next.rs"]
mod aliases_claude_next;
#[path = "aliases_opus.rs"]
pub mod aliases_opus;

/// Resolve a short model alias to the full Bedrock model ID.
///
/// Accepts both human-friendly aliases (`claude-opus-4-6`) and canonical
/// Bedrock inference-profile IDs; unknown inputs are returned unchanged.
///
/// # Arguments
///
/// * `model` — The user-supplied model identifier.
///
/// # Returns
///
/// A `&str` pointing at the canonical Bedrock model ID. When `model` already
/// is a full ID (or an unknown string), the returned slice aliases `model`.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::model_identity::bedrock::resolve_model_id;
///
/// assert_eq!(resolve_model_id("claude-opus-4"), "us.anthropic.claude-opus-4-20250514-v1:0");
/// assert_eq!(resolve_model_id("nova-lite"), "amazon.nova-lite-v1:0");
/// // sonnet-4-6 is an unsuffixed inference profile; the `-v1:0` form 400s.
/// assert_eq!(resolve_model_id("claude-sonnet-4-6"), "us.anthropic.claude-sonnet-4-6");
/// assert_eq!(
///     resolve_model_id("us.anthropic.claude-sonnet-4-6"),
///     "us.anthropic.claude-sonnet-4-6"
/// );
/// // fable must use the invokable `us.` profile, not the 500-returning `global.`
/// assert_eq!(resolve_model_id("fable"), "us.anthropic.claude-fable-5");
/// assert_eq!(
///     resolve_model_id("us.anthropic.claude-fable-5"),
///     "us.anthropic.claude-fable-5"
/// );
/// let full = "us.anthropic.claude-opus-4-6-v1";
/// assert_eq!(resolve_model_id(full), full);
/// ```
pub fn resolve_model_id(model: &str) -> &str {
    aliases_claude_next::resolve(model)
        .or_else(|| super::claude::resolve(model))
        .or_else(|| super::amazon_meta_mistral::resolve(model))
        .or_else(|| super::other::resolve(model))
        .or_else(|| super::aliases_openai::resolve_openai_alias(model))
        .unwrap_or(model)
}
