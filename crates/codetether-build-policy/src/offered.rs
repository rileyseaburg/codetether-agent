//! Established offered language markers for build-mode decisions.

/// Apply the established assistant offered next step rule.
/// # Arguments
/// Receives the message, tool or workspace inputs named in the signature.
/// # Returns
/// The decision or matching paths without executing tools.
/// # Examples
/// ```
/// assert!(codetether_build_policy::assistant_offered_next_step("next I can patch it"));
/// ```
pub fn assistant_offered_next_step(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let offer_markers = [
        "if you want",
        "want me to",
        "should i",
        "next i can",
        "i can also",
        "i'm ready to",
        "i am ready to",
    ];
    let action_markers = [
        "patch",
        "add",
        "update",
        "edit",
        "change",
        "fix",
        "implement",
        "style",
        "tighten",
        "apply",
        "refactor",
    ];
    offer_markers.iter().any(|m| lower.contains(m))
        && action_markers.iter().any(|m| lower.contains(m))
}
