//! Consistent ON/OFF labels and preference status messages.
/// Return the uppercase label for a boolean preference.
/// # Arguments
/// * `enabled` - Current preference.
/// # Returns
/// ON when enabled and OFF otherwise.
/// # Examples
/// ```
/// assert_eq!(codetether_settings_policy::on_off_label(true), "ON");
/// ```
pub fn on_off_label(enabled: bool) -> &'static str {
    if enabled { "ON" } else { "OFF" }
}
/// Describe the current preference value.
/// # Arguments
/// * `enabled` - Current preference.
/// # Returns
/// The existing TUI status text.
/// # Examples
/// ```
/// assert!(!codetether_settings_policy::autocomplete_status_message(false).is_empty());
/// ```
pub fn autocomplete_status_message(enabled: bool) -> String {
    format!("TUI slash autocomplete: {}", on_off_label(enabled))
}
/// Describe the current preference value.
/// # Arguments
/// * `enabled` - Current preference.
/// # Returns
/// The existing TUI status text.
/// # Examples
/// ```
/// assert!(!codetether_settings_policy::worktree_status_message(false).is_empty());
/// ```
pub fn worktree_status_message(enabled: bool) -> String {
    format!("TUI worktree isolation: {}", on_off_label(enabled))
}
/// Describe the current preference value.
/// # Arguments
/// * `enabled` - Current preference.
/// # Returns
/// The existing TUI status text.
/// # Examples
/// ```
/// assert!(!codetether_settings_policy::network_access_status_message(false).is_empty());
/// ```
pub fn network_access_status_message(enabled: bool) -> String {
    format!("TUI network access: {}", on_off_label(enabled))
}
/// Describe the current preference value.
/// # Arguments
/// * `enabled` - Current preference.
/// # Returns
/// The existing TUI status text.
/// # Examples
/// ```
/// assert!(!codetether_settings_policy::auto_apply_status_message(false).is_empty());
/// ```
/// Human-readable status line describing the auto-apply edit state.
pub fn auto_apply_status_message(enabled: bool) -> String {
    format!("TUI edit auto-apply: {}", on_off_label(enabled))
}
