//! Wire-level reasoning efforts accepted by OpenRouter.

/// Efforts OpenRouter accepts for `reasoning.effort`.
///
/// Taken verbatim from the router's own 400 response, which enumerates the
/// allowed set when given an invalid value:
/// `expected one of "max"|"xhigh"|"high"|"medium"|"low"|"minimal"|"none"`.
pub const LEVELS: &[&str] = &["none", "minimal", "low", "medium", "high", "xhigh", "max"];

/// Returns the canonical wire value for a user-supplied effort label.
///
/// Input is trimmed and lowercased so configuration and TUI values agree.
/// Returns `None` for anything OpenRouter would reject, letting callers omit
/// the field instead of sending a request that fails with HTTP 400.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::openrouter::reasoning_levels::normalize;
///
/// assert_eq!(normalize(" HIGH "), Some("high"));
/// assert_eq!(normalize("bogus"), None);
/// ```
pub fn normalize(value: &str) -> Option<&'static str> {
    let lowered = value.trim().to_ascii_lowercase();
    LEVELS.iter().copied().find(|level| *level == lowered)
}

/// Whether a model refuses `reasoning.effort: "none"`.
///
/// Grok 4.5 and 4.6 answer `Reasoning is mandatory for this endpoint and
/// cannot be disabled` (HTTP 400). Older Grok builds accept `none`, so the
/// check is version-specific rather than family-wide.
pub fn requires_reasoning(model: &str) -> bool {
    let lowered = model.to_ascii_lowercase();
    ["grok-4.5", "grok-4.6"]
        .iter()
        .any(|family| lowered.contains(family))
}

#[cfg(test)]
#[path = "reasoning_levels_tests.rs"]
mod tests;
