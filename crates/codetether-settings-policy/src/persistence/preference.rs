//! Boolean preference identity and save-result labels.
/// Preferences synchronized between interactive state and session metadata.
/// # Examples
/// ```
/// use codetether_settings_policy::persistence::Preference;
/// assert_eq!(Preference::Network.status(true), "TUI network access: ON");
/// ```
#[derive(Clone, Copy)]
pub enum Preference {
    /// Automatically apply accepted edits.
    AutoApply,
    /// Permit network access in sandboxed commands.
    Network,
    /// Enable slash-command autocomplete.
    Autocomplete,
    /// Enable worktree isolation.
    Worktree,
}
impl Preference {
    /// Return the existing status message for this preference.
    pub fn status(self, enabled: bool) -> String {
        match self {
            Self::AutoApply => crate::auto_apply_status_message(enabled),
            Self::Network => crate::network_access_status_message(enabled),
            Self::Autocomplete => crate::autocomplete_status_message(enabled),
            Self::Worktree => crate::worktree_status_message(enabled),
        }
    }
}
