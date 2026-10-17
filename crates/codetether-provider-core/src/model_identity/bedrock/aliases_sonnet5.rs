//! Bedrock alias resolution for Claude Sonnet 5.x.
//!
//! Observed live against bedrock-runtime: `us.anthropic.claude-sonnet-5-5`
//! is rejected as an invalid identifier and bare `anthropic.claude-sonnet-5-5`
//! requires an inference profile, while `global.anthropic.claude-sonnet-5-5`
//! is accepted. Sonnet 5 has both `us.` and `global.` profiles, so those IDs
//! pass through unchanged.

/// Resolve a Sonnet 5.5 alias to its global inference profile.
///
/// # Returns
///
/// `Some(canonical_id)` for a known Sonnet 5.5 alias, otherwise `None` so
/// the caller can continue matching other families.
pub(super) fn resolve_sonnet5_alias(model: &str) -> Option<&'static str> {
    match model {
        "claude-sonnet-5.5"
        | "claude-sonnet-5-5"
        | "claude-5.5-sonnet"
        | "sonnet-5.5"
        | "anthropic.claude-sonnet-5-5"
        | "us.anthropic.claude-sonnet-5-5" => Some("global.anthropic.claude-sonnet-5-5"),
        _ => None,
    }
}

#[cfg(test)]
#[path = "aliases_sonnet5_tests.rs"]
mod tests;
