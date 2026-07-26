//! Bedrock alias resolution for the Anthropic Claude Opus family.
//!
//! Split out of [`super::aliases`] so new Opus generations can be added
//! without growing the main alias dispatch table.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::aliases_opus::resolve_opus_alias;
//!
//! assert_eq!(
//!     resolve_opus_alias("claude-opus-5"),
//!     Some("global.anthropic.claude-opus-5")
//! );
//! assert_eq!(resolve_opus_alias("claude-sonnet-4-6"), None);
//! ```

/// Resolve a Claude Opus alias to its canonical Bedrock model ID.
///
/// # Arguments
///
/// * `model` — User-supplied model identifier.
///
/// # Returns
///
/// `Some(canonical_id)` for a known Opus alias, otherwise `None` so the
/// caller can continue matching other families.
pub fn resolve_opus_alias(model: &str) -> Option<&'static str> {
    Some(match model {
        // Opus 5 is served from the global inference profile (verified live
        // against bedrock-runtime us-east-1).
        "claude-opus-5" | "claude-5-opus" | "opus-5" => "global.anthropic.claude-opus-5",
        "claude-opus-4.7" | "claude-opus-4-7" | "claude-4.7-opus" => "us.anthropic.claude-opus-4-7",
        "claude-opus-4.6"
        | "claude-opus-4-6"
        | "claude-4.6-opus"
        | "us.anthropic.claude-opus-4-6" => "us.anthropic.claude-opus-4-6-v1",
        "claude-opus-4.5" | "claude-4.5-opus" | "us.anthropic.claude-opus-4-5" => {
            "us.anthropic.claude-opus-4-5-20251101-v1:0"
        }
        "claude-opus-4.1" | "claude-4.1-opus" | "us.anthropic.claude-opus-4-1" => {
            "us.anthropic.claude-opus-4-1-20250805-v1:0"
        }
        "claude-opus-4" | "claude-4-opus" | "us.anthropic.claude-opus-4" => {
            "us.anthropic.claude-opus-4-20250514-v1:0"
        }
        _ => return None,
    })
}

#[cfg(test)]
#[path = "aliases_opus_tests.rs"]
mod tests;
