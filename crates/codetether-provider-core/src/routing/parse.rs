//! Provider/model path parsing with nested model namespaces.
//!
//! # Examples
//!
//! ```rust
//! use codetether_provider_core::routing::parse_model_string;
//!
//! assert_eq!(parse_model_string("openai/gpt-4o"), (Some("openai"), "gpt-4o"));
//! assert_eq!(parse_model_string("gpt-4o"), (None, "gpt-4o"));
//! assert_eq!(
//!     parse_model_string("openrouter/openai/gpt-5-codex"),
//!     (Some("openrouter"), "openai/gpt-5-codex")
//! );
//! ```

/// Parse a model string into `(provider, model)`.
///
/// Supports:
/// - `provider/model` → (`provider`, `model`)
/// - nested provider namespaces like `openai-codex/gpt-5.5`
/// - OpenRouter-style paths like `openrouter/openai/gpt-5-codex`
///
/// For three-or-more path segments, the first segment is the provider and
/// the remainder is the model ID.
///
/// # Arguments
/// * `s` - Bare model name or provider-qualified model path.
/// # Returns
/// The optional first path segment and the unmodified remaining model path.
pub fn parse_model_string(s: &str) -> (Option<&str>, &str) {
    let mut parts = s.split('/');
    let Some(first) = parts.next() else {
        return (None, s);
    };

    let remainder_start = first.len();
    if remainder_start >= s.len() {
        return (None, s);
    }

    let remainder = &s[remainder_start + 1..];
    (Some(first), remainder)
}
