//! Access-mode labels and deterministic cycle order.
use codetether_config::AccessMode;
/// Map the current access mode to its successor.
/// # Arguments
/// * `mode` - Current access mode.
/// # Returns
/// The next mode in the ask, approve, full cycle.
/// # Examples
/// ```
/// assert!(matches!(codetether_settings_policy::next_access_mode(codetether_config::AccessMode::Ask), codetether_config::AccessMode::Approve));
/// ```
pub fn next_access_mode(mode: AccessMode) -> AccessMode {
    match mode {
        AccessMode::Ask => AccessMode::Approve,
        AccessMode::Approve => AccessMode::Full,
        AccessMode::Full => AccessMode::Ask,
    }
}
/// Map the current access mode to its label.
/// # Arguments
/// * `mode` - Current access mode.
/// # Returns
/// The lowercase access-mode label.
/// # Examples
/// ```
/// assert_eq!(codetether_settings_policy::access_label(codetether_config::AccessMode::Ask), "ask");
/// ```
/// Human-readable label for an access mode.
pub fn access_label(mode: AccessMode) -> &'static str {
    match mode {
        AccessMode::Ask => "ask",
        AccessMode::Approve => "approve",
        AccessMode::Full => "full",
    }
}
