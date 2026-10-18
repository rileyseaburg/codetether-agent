//! Established proposed language markers for build-mode decisions.

/// Apply the established looks like proposed change rule.
/// # Arguments
/// Receives the message, tool or workspace inputs named in the signature.
/// # Returns
/// The decision or matching paths without executing tools.
/// # Examples
/// ```
/// assert!(codetether_build_policy::looks_like_proposed_change("replace the handler"));
/// ```
pub fn looks_like_proposed_change(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let markers = [
        "use this exact block",
        "now uses",
        "apply",
        "replace",
        "patch",
        "edit",
        "change",
        "update",
        "fix",
    ];
    markers.iter().any(|m| lower.contains(m))
}
