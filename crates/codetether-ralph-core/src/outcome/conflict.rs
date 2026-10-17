//! Preserve the conflict resolver success-marker policy.
/// Classify resolver output using its existing text markers.
///
/// # Arguments
/// `output` is the resolver response.
/// # Returns
/// Whether the response reports resolved conflicts.
/// # Examples
/// ```
/// assert!(codetether_ralph_core::outcome::conflict_resolved("CONFLICTS_RESOLVED"));
/// assert!(!codetether_ralph_core::outcome::conflict_resolved("UNRESOLVED"));
/// ```
pub fn conflict_resolved(output: &str) -> bool {
    output.contains("CONFLICTS_RESOLVED")
        || (output.contains("resolved") && !output.contains("UNRESOLVED"))
}
