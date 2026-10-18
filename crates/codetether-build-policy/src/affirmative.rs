//! Established affirmative language markers for build-mode decisions.

/// Apply the established is affirmative build followup rule.
/// # Arguments
/// Receives the message, tool or workspace inputs named in the signature.
/// # Returns
/// The decision or matching paths without executing tools.
/// # Examples
/// ```
/// assert!(codetether_build_policy::is_affirmative_build_followup("yes"));
/// ```
pub fn is_affirmative_build_followup(text: &str) -> bool {
    let lower = text.trim().to_ascii_lowercase();
    let markers = [
        "yes",
        "yep",
        "yeah",
        "do it",
        "go ahead",
        "proceed",
        "use the edit",
        "use edit",
        "apply it",
        "ship it",
        "fix it",
    ];
    markers
        .iter()
        .any(|m| lower == *m || lower.starts_with(&format!("{m} ")))
}
