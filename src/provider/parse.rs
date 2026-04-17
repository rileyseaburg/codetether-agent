//! Model-string parsing and provider URL normalization helpers.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::parse_model_string;
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
/// - nested provider namespaces like `openai-codex/gpt-5.4`
/// - OpenRouter-style paths like `openrouter/openai/gpt-5-codex`
///
/// For three-or-more path segments, the first segment is the provider and
/// the remainder is the model ID.
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

/// Normalize MiniMax base URLs to the Anthropic-compatible endpoint.
///
/// MiniMax's docs recommend using the `/anthropic` path for their
/// Anthropic-compatible harness. This rewrites the legacy `/v1` path.
pub(crate) fn normalize_minimax_anthropic_base_url(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.eq_ignore_ascii_case("https://api.minimax.io/v1") {
        "https://api.minimax.io/anthropic".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_provider_model() {
        assert_eq!(parse_model_string("openai/gpt-4o"), (Some("openai"), "gpt-4o"));
    }

    #[test]
    fn parses_bare_model() {
        assert_eq!(parse_model_string("gpt-4o"), (None, "gpt-4o"));
    }

    #[test]
    fn parses_nested_provider() {
        assert_eq!(
            parse_model_string("openrouter/openai/gpt-5-codex"),
            (Some("openrouter"), "openai/gpt-5-codex")
        );
    }

    #[test]
    fn normalizes_minimax_url() {
        assert_eq!(
            normalize_minimax_anthropic_base_url("https://api.minimax.io/v1"),
            "https://api.minimax.io/anthropic"
        );
        assert_eq!(
            normalize_minimax_anthropic_base_url("https://custom.host/custom"),
            "https://custom.host/custom"
        );
    }
}
