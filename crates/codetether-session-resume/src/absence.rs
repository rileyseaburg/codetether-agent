//! Distinguish expected absence of history from an unexpected scan failure.

/// Classify the existing normal no-history messages.
///
/// # Arguments
/// * `reason` — Session scan error or fresh-start reason.
///
/// # Returns
/// Whether the reason should start fresh without an unexpected-failure banner.
///
/// # Examples
/// ```
/// assert!(codetether_session_resume::is_expected_absence("no session"));
/// ```
pub fn is_expected_absence(reason: &str) -> bool {
    reason.contains("requested fresh session")
        || reason.contains("no session")
        || reason.contains("No such file")
        || reason.contains("not found")
}
